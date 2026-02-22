# keriox Usage Patterns Reference

## 1. Identity Lifecycle

### 1.1 Inception (Low-Level)

```rust
let key_config = KeyConfig::new(
    keys, NextKeysData::new(SignatureThreshold::Simple(2), next_key_hashes),
    Some(SignatureThreshold::Simple(2)));
let icp_msg = InceptionEvent::new(key_config, None, None)
    .incept_self_addressing(HashFunctionCode::Blake3_256.into(), SerializationFormats::JSON)?;
let signed = icp_msg.sign(vec![IndexedSignature::new_both_same(
    SelfSigningPrefix::Ed25519Sha512(private_key.sign_ed(&icp_msg.encode()?)?), 0)], None, None);
let state = IdentifierState::default().apply(&signed)?;
```

Self-addressing: dummy prefix -> serialize -> hash -> hash becomes prefix + digest.

### 1.2 Inception (EventMsgBuilder)

```rust
let icp = EventMsgBuilder::new(EventTypeTag::Icp)
    .with_keys(keys).with_threshold(&SignatureThreshold::Simple(2))
    .with_next_keys(next_keys).with_next_threshold(&SignatureThreshold::Simple(2))
    .with_witness_list(&witnesses).with_witness_threshold(&SignatureThreshold::Simple(1))
    .build()?;
```

Defaults: self-addressing prefix if default, JSON, Blake3-256, Simple(1).

### 1.3 Rotation

```rust
let rot = EventMsgBuilder::new(EventTypeTag::Rot)
    .with_prefix(&id).with_sn(1).with_previous_event(&prev_digest)
    .with_keys(new_keys).with_next_keys(next_next_keys)
    .with_witness_to_add(&new).with_witness_to_remove(&old)
    .with_witness_threshold(&SignatureThreshold::Simple(1)).build()?;
```

### 1.4 Interaction (Anchoring)

```rust
let ixn = EventMsgBuilder::new(EventTypeTag::Ixn)
    .with_prefix(&id).with_sn(2).with_previous_event(&prev_digest)
    .with_seal(vec![Seal::Digest(DigestSeal::new(said))]).build()?;
```

Seals: `Seal::Digest`, `Seal::Event(EventSeal)`, `Seal::Location(LocationSeal)`.

### 1.5 Delegated Inception

```rust
let dip = EventMsgBuilder::new(EventTypeTag::Dip)
    .with_keys(keys).with_next_keys(next_keys).with_delegator(&delegator_prefix)
    .with_witness_list(&witnesses).with_witness_threshold(&SignatureThreshold::Simple(1)).build()?;
```

Delegate creates `dip` -> includes `delegator_seal: Some(SourceSeal)` -> delegator anchors with `ixn`+`EventSeal` -> until anchored, sits in `DelegationEscrow`.

### 1.6 Group Multisig

```rust
let (group_icp, exchanges) = identifier.incept_group(
    participants, sig_threshold, next_threshold, witnesses, wit_threshold, delegator)?;
let prefix = identifier.finalize_group_incept(
    group_icp.as_bytes(), sig, signed_exchanges).await?;
identifier.notify_witnesses().await?;
```

Deterministic event -> each signs at own index -> `/fwd` (ForwardTopic::Multisig) -> PartiallySignedEscrow -> min-index sends to witness.

### 1.7 CryptoBox

```rust
let mut cb = CryptoBox::new()?; // random Ed25519 current + next
cb.sign(b"msg")?; cb.public_key(); cb.next_public_key();
cb.rotate()?; // current = old next, next = fresh
```

---

## 2. Event Processing

### 2.1 Pipeline

```
Processor::process(msg)
  Notice::Event -> validate
    Ok -> add_kel_finalized_event + notify(KeyEventAdded)
    OutOfOrder -> notify(OutOfOrder)       NotEnoughRct -> notify(PartiallyWitnessed)
    NotEnoughSig -> notify(PartiallySigned) Duplicate -> notify(DupliciousEvent)
    MissingDeleg -> notify(MissingDelegatingEvent)
  Notice::NontransferableRct -> store or ReceiptOutOfOrder
  Notice::TransferableRct    -> store or TransReceiptOutOfOrder
```

### 2.2 Parsing

```rust
let messages: Vec<Message> = parse_event_stream(stream)?;
let notices: Vec<Notice> = parse_notice_stream(stream)?;
```

### 2.3 Escrow Promotion

| Escrow | Listens For | Promotes On |
|--------|-------------|-------------|
| MaybeOutOfOrderEscrow | OutOfOrder, KeyEventAdded | Predecessor in KEL |
| PartiallySignedEscrow | PartiallySigned | Sigs meet threshold |
| PartiallyWitnessedEscrow | PartiallyWitnessed, ReceiptOutOfOrder | Receipts meet threshold |
| DelegationEscrow | MissingDelegatingEvent, KeyEventAdded | Delegator anchor added |
| DuplicitousEvents | DupliciousEvent | Never (storage only) |

Each `KeyEventAdded` cascades -- promoting one may trigger further promotions.

### 2.4 Notification Bus

```rust
let bus = NotificationBus::new();
let bus = NotificationBus::from_dispatch(Arc::new(sqs_dispatch)); // serverless
bus.register_observer(escrow, vec![JustNotification::OutOfOrder, JustNotification::KeyEventAdded]);
```

### 2.5 Receipts

```rust
let receipt = ReceiptBuilder::default()
    .with_receipted_event(event).with_format(SerializationFormats::JSON).build()?;
let sig = SelfSigningPrefix::Ed25519Sha512(witness_signer.sign(&receipt.encode()?)?);
SignedNontransferableReceipt::new(&receipt, vec![Nontransferable::Couplet(vec![(wit_prefix, sig)])]);
```

### 2.6 State

```rust
let state = IdentifierState::default();
let state = icp.apply_to(state)?; // then rot, ixn...
let state = compute_state(db.clone(), &prefix); // replay from DB
```

### 2.7 Wiring

```rust
let (bus, escrows) = default_escrow_bus(db.clone(), EscrowConfig::default(), None);
let processor = BasicProcessor::new(db.clone(), Some(bus));
// OOO + PartiallySigned + PartiallyWitnessed + Delegation + Duplicitous. ReplyEscrow separate.
```

---

## 3. Database

### 3.1 In-Memory

```rust
let db = Arc::new(MemoryDatabase::new()); // RwLock<HashMap>, no persistence
```

### 3.2 ReDB

```rust
let db = RedbDatabase::new(Path::new("/path/to/db.redb"))?;
let txn = db.begin_write()?;
let mode = WriteTxnMode::UseExisting(&txn);
self.update_key_state(&mode, &event)?;
self.log_db.log_event(&mode, &event)?;
txn.commit()?;
```

### 3.3 DynamoDB

```rust
let db = DynamoDatabase::new(client, DynamoConfig::new("prod-", "us-east-1"));
let db = DynamoDatabase::from_aws_config(DynamoConfig::new("prod-", "us-east-1")).await;
```

Tables: `kel` (PK=aid, SK=sn zero-padded), `states` (PK=aid), `events` (PK=digest), `escrows` (PK=name#aid, SK=sn#digest+TTL), `replies` (PK=aid, SK=signer). Conditional writes prevent duplicates.

### 3.4 New Backend Traits

1. **`LogDatabase`** -- store by SAID: `log_event`, `get_signed_event`, `get_signatures`
2. **`SequencedEventDatabase`** -- `(prefix, sn)` -> digests: `insert`, `get`, `remove`
3. **`EventDatabase`** -- compose Log+KEL+state. `add_kel_finalized_event`: read -> apply -> save -> log -> index
4. **`EscrowDatabase`** -- compose Sequenced+Log: `insert`, `get`, `remove`, `contains`
5. **`EscrowCreator`** -- factory: `create_escrow_db(table_name)`

---

## 4. OOBI and Discovery

```rust
let loc = LocationScheme::new(eid, Scheme::Http, url.parse()?);
let end_role = EndRole { cid: controller, role: Role::Witness, eid: witness };
// Oobi::Location(loc) or Oobi::EndRole(end_role)

identifier.resolve_oobi(&oobi).await?;
identifier.send_oobi_to_watcher(&other_id, &oobi).await?;

let rpy = identifier.add_watcher(watcher_id)?;
identifier.finalize_add_watcher(rpy.as_bytes(), sig).await?;

let rpy = generate_end_role(&cid, &wid, Role::Watcher, true);
let locs = oobi_manager.get_loc_scheme(&prefix)?;
let roles = oobi_manager.get_end_role(&prefix, Role::Witness)?;
```

---

## 5. Queries and Mailbox

### 5.1 Query Construction

```rust
let query = QueryEvent::new_query(QueryRoute::Ksn {
    reply_route: String::new(),
    args: LogsQueryArgs { i: target, s: None, limit: None, src: Some(source) },
}, SerializationFormats::JSON, HashFunctionCode::Blake3_256);
// Also: QueryRoute::Logs { args: { s: Some(0), limit: Some(100), .. } }
```

Server: `process_signed_query(qry, &storage)?` returns `ReplyType::Ksn|Kel|Mbx`.

### 5.2 Mailbox Polling

```rust
let queries = identifier.query_mailbox(&identifier.id(), &[wit1, wit2])?;
for qry in queries {
    let sig = SelfSigningPrefix::Ed25519Sha512(km.sign(&qry.encode()?)?);
    let actions = identifier.finalize_query_mailbox(vec![(qry, sig)]).await?;
    // ActionRequired::MultisigRequest(..) | DelegationRequest(..)
}
```

BADA: `bada_logic(&new, &old)?` -- transferable: sn then timestamp; nontransferable: timestamp. Stale -> `Err(StaleRpy)`.

---

## 6. TEL Lifecycle

### 6.1 Low-Level

```rust
let vcp = tel.make_inception_event(issuer, vec![Config::NoBackers], 0, vec![])?;
let seal = Seal::Digest(DigestSeal::new(vcp.get_digest()?));
// Anchor seal in KEL ixn, then:
let verifiable = VerifiableEvent::new(vcp, AttachedSourceSeal::new(ixn_sn, ixn_digest));
tel.processor.process(verifiable)?;

let iss = tel.make_issuance_event(&registry_id, vc_digest)?;
let rev = tel.make_revoke_event(&registry_id, &vc_hash)?;
// Same: anchor in KEL ixn -> VerifiableEvent -> process
```

State: `tel.get_vc_state(&hash)?` returns `NotIssued | Issued(digest) | Revoked`.

### 6.2 Via keri-controller

```rust
let (registry_id, ixn) = identifier.incept_registry()?;
identifier.finalize_incept_registry(&ixn.encode()?, sig).await?;
let (vc_hash, ixn) = identifier.issue(credential_digest)?;
identifier.finalize_issue(&ixn.encode()?, sig).await?;
identifier.finalize_revoke(&identifier.revoke(&sai)?, sig).await?;
```

### 6.3 TEL Escrow Setup

```rust
let (tel_bus, missing_issuer, _, _) =
    default_escrow_bus(tel_db.clone(), kel_storage.clone(), escrow_db)?;
keri_processor.register_observer(missing_issuer, &vec![JustNotification::KeyEventAdded])?;
let tel = Tel::new(Arc::new(TelEventStorage::new(tel_db)), kel_storage, Some(tel_bus));
```

---

## 7. Witness and Watcher

### 7.1 Witness

```rust
let listener = WitnessListener::setup(public_url, &db_path, seed, escrow_config)?;
listener.listen_http("0.0.0.0:3236").await?;
```

Creates: Signer, RedbDatabase, WitnessProcessor (3 escrows), WitnessReceiptGenerator, TEL, OobiManager. `NotEnoughRct` is NOT an error (witness IS the receipt source).

Endpoints: GET `/introduce` (OOBI), `/oobi/{id}`. POST `/process` (CESR events), `/query` (KEL/KSN/mailbox), `/register` (OOBIs), `/forward` (multisig/delegation), `/process/tel`, `/query/tel`.

### 7.2 Watcher

```rust
let listener = WatcherListener::new(WatcherConfig {
    public_address, db_path, priv_key: None,
    transport: Box::new(DefaultTransport::new()),
    tel_transport: Box::new(HttpTelTransport),
    tel_storage_path, escrow_config: EscrowConfig::default(),
})?;
listener.resolve_initial_oobis(&[witness_oobi]).await?;
listener.listen_http("0.0.0.0:3236").await?;
```

Verifies + stores but does NOT generate receipts. Trusted cache with background update tasks.

Both use: `BasicPrefix::Ed25519NT`, ReDB, actix-web, Figment (`WITNESS_*`/`WATCHER_*` env).

---

## 8. SDK Two-Phase Flow

Pattern: Generate -> Sign externally (keys never touch SDK) -> Finalize.

### keri-sdk (generic, no networking)

```rust
let controller = Controller::new(event_db, tel_db);
let icp = controller.incept(vec![pk], vec![npk])?;
let identifier = controller.finalize_incept(icp.as_bytes(), &sig)?;
```

### keri-controller (full networking)

```rust
let controller = Controller::new(ControllerConfig {
    db_path, initial_oobis: vec![], escrow_config: EscrowConfig::default(),
    transport: Box::new(DefaultTransport::new()), tel_transport: Box::new(HTTPTelTransport),
})?;
let icp = controller.incept(vec![pk], vec![npk], vec![witness_loc], 1).await?;
let mut id = controller.finalize_incept(icp.as_bytes(), &sig)?;
id.notify_witnesses().await?;
```

### Two-Phase Flow Table

| Operation | Generate | Finalize |
|-----------|----------|----------|
| Inception | `controller.incept(..)` | `controller.finalize_incept(..)` |
| Rotation | `identifier.rotate(..)` | `identifier.finalize_rotate(..)` |
| Anchor | `identifier.anchor(..)` | `identifier.finalize_anchor(..)` |
| Add Watcher | `identifier.add_watcher(..)` | `identifier.finalize_add_watcher(..)` |
| Group Inception | `identifier.incept_group(..)` | `identifier.finalize_group_incept(..)` |
| Group Event | (exchange via mailbox) | `identifier.finalize_group_event(..)` |
| Registry | `identifier.incept_registry()` | `identifier.finalize_incept_registry(..)` |
| Issue | `identifier.issue(..)` | `identifier.finalize_issue(..)` |
| Revoke | `identifier.revoke(..)` | `identifier.finalize_revoke(..)` |
| Query KEL/KSN | `identifier.query_watchers(..)` | `identifier.finalize_query(..)` |
| Query Mailbox | `identifier.query_mailbox(..)` | `identifier.finalize_query_mailbox(..)` |
| Query TEL | `identifier.query_tel(..)` | `identifier.finalize_query_tel(..)` |

KnownEvents = KeriRuntime + OobiManager + PartiallyWitnessedEscrow + Tel. Concrete ReDB vs generic.

---

## 9. Signing and Verification

```rust
let signer = Signer::new();                 // random Ed25519
let signer = Signer::new_with_seed(&seed)?; // from CESR seed
let bp = BasicPrefix::Ed25519(pub_key);      // D = transferable
let bp = BasicPrefix::Ed25519NT(pub_key);    // B = non-transferable
// ECDSA: 1AAB transferable, 1AAA non-transferable

let sig = SelfSigningPrefix::Ed25519Sha512(signer.sign(b"msg")?); // 0B, 64B
IndexedSignature::new_both_same(sig, 0);
IndexedSignature::new_both_diffrent(sig, cur_idx, next_idx);
IndexedSignature::new_current_only(sig, 0);

key_prefix.verify(&msg, &sig)?;            // single key
key_config.verify(&msg, &indexed_sigs)?;   // threshold
identifier.verify_from_cesr(cesr_stream)?; // keri-controller
// Ed25519 <-> Ed25519Sha512, ECDSA <-> ECDSAsecp256k1Sha256

SignatureThreshold::Simple(2);
SignatureThreshold::single_weighted(vec![(1,2),(1,2),(1,2)]);
SignatureThreshold::multi_weighted(vec![clause1, clause2]);

let msg: Message = Message::try_from(parse(stream)?.1)?;
let bytes = msg.to_cesr()?;
let prefix: BasicPrefix = "DErocg...".parse()?; // prefix.to_str()
```

---

## 10. Network Communication

### Transport

Key methods (all async): `send_message(loc, msg)`, `send_query(loc, qry)`, `request_loc_scheme(loc)`, `resolve_oobi(loc, oobi)`.

### HTTP Routing

Notice -> POST `/process`, Reply -> POST `/register`, Exchange -> POST `/forward`, Query -> POST `/query`, Location -> GET `/oobi/{eid}`, EndRole -> GET `/oobi/{cid}/{role}/{eid}`, Resolve -> POST `/resolve`.

### TestTransport

```rust
let mut actors: TestActorMap = HashMap::new();
actors.insert((url::Host::Domain("witness1".into()), 3236), Arc::new(actor));
let transport = TestTransport::new(actors);
```

### SQS Dispatch

```rust
let dispatch = SqsDispatch::from_aws_config(queue_url, region, None).await;
let bus = NotificationBus::from_dispatch(Arc::new(dispatch));
// register_observer is no-op -- subscribers at infra level
```

### Communication Layer

```rust
let comm = Communication::new(known_events, Box::new(DefaultTransport::new()), Box::new(HTTPTelTransport));
comm.resolve_oobi(&oobi).await?;
comm.publish(witness_prefixes, &signed_event).await?;
```

---

## Anti-Patterns

**Event Construction**
- Manually setting inception prefix -> use `incept_self_addressing()` or leave default
- Rot/Ixn without `with_previous_event` -> always chain to prior digest
- Rotation keys don't match commitment -> MUST hash-match `next_keys_data`

**Processing**
- Escrow = error -> escrow = state, return 202, let bus promote
- Missing escrow observers -> use `default_escrow_bus()` before processing
- Witness rejects NotEnoughReceipts -> `witness_processing_strategy` accepts (witness IS receipt source)

**Database**
- `WriteTxnMode::CreateNew` multi-step -> `begin_write()` + `UseExisting` + `commit()`
- Non-atomic `add_kel_finalized_event` -> state+log+index must be atomic

**Two-Phase**
- Signing inside SDK -> generate, sign externally, finalize
- Skip `notify_witnesses()` -> witnesses need event to generate receipts
- keri-sdk for networking -> use keri-controller

**TEL**
- TEL without KEL anchor -> always ixn + `AttachedSourceSeal`
- No cross-bus bridge -> register `missing_issuer` on KEL bus for `KeyEventAdded`

**Signing**
- Ed25519 + ECDSA sig -> types must match
- `new_both_same` for different cur/next positions -> `new_both_diffrent(sig, cur, next)`
