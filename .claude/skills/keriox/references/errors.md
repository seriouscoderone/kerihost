# keriox Error Reference

Compact reference for all error types, escrow routing, and cross-crate conversions.

---

## 1. keri_core::Error (top-level)

`keriox_core/src/error/mod.rs`

| Variant | Inner | When Produced | Escrow? |
|---------|-------|---------------|---------|
| `SerializationError` | String | CESR/JSON encode failure | No |
| `SemanticError` | String | Generic semantic validation (wrong type, missing field) | No |
| `FaultySignatureVerification` | -- | Signature does not match pubkey+data | No |
| `EventOutOfOrderError` | -- | Event sn != expected next sn | **Yes** -> `OutOfOrder` -> `MaybeOutOfOrderEscrow` |
| `MissingDelegatorSealError` | IdentifierPrefix | Delegated event lacks delegator seal | **Yes** -> `MissingDelegatingEvent` -> `DelegationEscrow` |
| `MissingDelegatingEventError` | -- | Delegator anchoring event not in DB | **Yes** -> `MissingDelegatingEvent` -> `DelegationEscrow` |
| `EventDuplicateError` | -- | Different event at same sn in KEL | **Yes** -> `DupliciousEvent` -> `DuplicitousEvents` |
| `NotEnoughSigsError` | -- | Sigs below signing threshold | **Yes** -> `PartiallySigned` -> `PartiallySignedEscrow` |
| `NotEnoughReceiptsError` | -- | Receipts below witness threshold | **Yes** -> `PartiallyWitnessed` -> `PartiallyWitnessedEscrow` |
| `MissingEvent` | -- | Receipted event not in DB | **Yes** -> `ReceiptOutOfOrder` |
| `MissingSignatures` | -- | No signatures attached | No |
| `MissingSigner` | -- | Cannot determine signer | No |
| `UnknownSigner` | IdentifierPrefix | Signer state not in DB | No |
| `SignatureVerificationError` | -- | Crypto sig invalid (silent escrow removal) | No |
| `ReceiptVerificationError` | -- | Receipt sig invalid | No |
| `DeserializeError` | ParseError | CESR/JSON deser failure | No |
| `NotIndexedError` | -- | Expected indexed sig, got non-indexed | No |
| `IdentifierPresentError` | -- | Inception collision | No |
| `IncorrectDigest` | -- | SAID verification failed | No |
| `EventDigestError` | -- | Event missing digest field | No |
| `QueryError` | QueryError | Query errors (feature-gated) | No |
| `DbError` | -- | Generic DB failure | No |
| `EventGenerationError` | String | Event builder error | No |
| `PrefixModuleError` | prefix::error::Error | Prefix parse/derive error | No |
| `CesrError` | -- | CESR codec error | No |
| `VersionError` | -- | Version string parse error | No |
| `SAIError` | -- | Self-addressing identifier error | No |
| `SigningError` | -- | Signing operation failed | No |
| `KeyConfigError` | SignatureError | Key config validation error | No |
| `VerificationError` | VerificationError | Structured verification error | No |
| `SerdeSerError` | serializer_error::Error | Custom JSON serializer error | No |
| `MutArcKeyVaultError` / `MutexPoisoned` / `RwLockingError` | -- | Lock/mutex errors | No |
| `SledError` | -- | Legacy sled DB error | No |

---

## 2. Subsidiary Error Types (keri_core)

### SignatureError (`event/sections/key_config.rs`)

| Variant | When Produced |
|---------|---------------|
| `NotEnoughSigsError` | Sig indexes don't satisfy threshold |
| `DuplicateSignature` | Same key index appears twice |
| `TooManySignatures` | More sigs than keys |
| `MissingIndex` | Sig refs nonexistent key index |
| `WrongSignatureTypeError` | Sig algo != key algo |
| `WrongKeyTypeError` | Key type unsupported |

### VerificationError (`processor/validator.rs`)

| Variant | Inner | When Produced |
|---------|-------|---------------|
| `VerificationFailure` | -- | Sig bytes don't match |
| `SignatureError` | SignatureError | Threshold/index fail during verify |
| `NotEstablishment` | EventSeal | Signer refs non-establishment event |
| `MissingSignerId` | -- | No signer data |
| `MoreInfo(EventNotFound)` | EventSeal | Referenced event not in DB |
| `MoreInfo(UnknownIdentifier)` | IdentifierPrefix | Signer has no state |

### ParseError (`event_message/cesr_adapter.rs`)

| Variant | When Produced |
|---------|---------------|
| `CesrError(msg)` | `cesrox::parse` fails |
| `DeserializeError(msg)` | JSON/CBOR deser fails |
| `AttachmentError(msg)` | Attachment group parsing fails |
| `WrongEventType(msg)` | Event type tag mismatch |

### QueryError -- query module (`query/mod.rs`)

| Variant | When Produced |
|---------|---------------|
| `StaleKsn` | KSN reply older than accepted |
| `StaleRpy` | Reply has older sn/timestamp |
| `NoSavedReply` | No previous reply for comparison |
| `Error(msg)` | Generic |

### OobiError (`oobi/error.rs`)

| Variant | When Produced |
|---------|---------------|
| `Keri(e)` | Core KERI error during OOBI |
| `Db(msg)` | DB error during OOBI storage |
| `Parse(msg)` | OOBI message parse failure |
| `Query(qe)` | BADA logic failure |
| `SignerMismatch` | Reply signer != expected entity |
| `InvalidMessageType` | Wrong msg type for OOBI |

### ActorError (`actor/error.rs`)

| Variant | When Produced |
|---------|---------------|
| `TransportError` | Network failure (cfg oobi-manager) |
| `KeriError` | Core KERI error |
| `DbError` | DB failure |
| `OobiError` | OOBI failure (cfg oobi) |
| `QueryError` | Query failure |
| `ParseError` | CESR parse failure |
| `NoLocation` | No location for id |
| `WrongReplyRoute` | Wrong reply route |
| `MissingRole` | Role not found (cfg oobi) |
| `MissingSignerId` | No signer id |
| `SigningError` | Signing failure |
| `GeneralError` | Catch-all |
| `NotFound` | Identifier not found |
| `UnexpectedResponse` | Unexpected response |

**HTTP status mapping** (`ActorError::http_status_code()`):

| Pattern | HTTP |
|---------|------|
| `KeriError(DeserializeError \| IncorrectDigest)` | 400 |
| `KeriError(FaultySignatureVerification \| SignatureVerificationError)` | 403 |
| `OobiError(SignerMismatch)` | 401 |
| Everything else | 500 |

---

## 3. teliox::Error (`support/teliox`)

| Variant | Inner | When Produced | Escrow? |
|---------|-------|---------------|---------|
| `KeriError` | keri_core::Error | KEL-level error during TEL | No |
| `RedbError` | -- | Redb storage failure | No |
| `Generic` | String | Catch-all (also DynamoDB bridge) | No |
| `EncodingError` | String | TEL event CESR/CBOR encode failure | No |
| `EscrowDatabaseError` | String | Escrow DB failure | No |
| `MissingSealError` | -- | KEL event missing expected TEL seal | No |
| `MissingIssuerEventError` | -- | KEL event at seal.sn not in DB | **Yes** -> `MissingIssuer` -> `MissingIssuerEscrow` |
| `MissingRegistryError` | -- | Registry TEL state not found | **Yes** -> `MissingRegistry` -> `MissingRegistryEscrow` |
| `OutOfOrderError` | -- | TEL event sn != expected | **Yes** -> `OutOfOrder` -> `OutOfOrderEscrow` |
| `DigestsNotMatchError` | -- | KEL digest at seal.sn != seal.digest | No |
| `UnknownIdentifierError` | -- | No TEL state for id | No |
| `EventAlreadySavedError` | -- | Duplicate TEL event | No (returns Ok) |
| `RwLockingError` | -- | RwLock poisoned | No |

---

## 4. Controller Errors (`components/controller`) -- summary

`ControllerError` wraps: `RedbError`, `SendingError`, `ParseError`, `keri_core::Error` (as `EventProcessingError`), `teliox::Error` (as `TelError`), `MechanicsError`, `WatcherResponseError`, plus `NoLocationScheme{id,scheme}`, `QueryArgumentError(String)`, `CesrFormatError`, `FaultySignature`, `VerificationError(Vec<..>)`.

`MechanicsError` wraps: `SendingError`, `TransportError`, `keri_core::Error`, `ResponseProcessingError`, `BroadcastingError`, `OobiError`, plus `UnknownIdentifierError(id)`, `EventGenerationError(msg)`, `NotGroupParticipantError`, `InceptionError(msg)`, `WrongWitnessPrefixError`.

All use `#[from]` for automatic conversion chains.

---

## 5. Witness/Watcher Errors

### WitnessError (`components/witness`)

| Variant | Inner | When Produced |
|---------|-------|---------------|
| `KeriError` | keri_core::Error | Core processing failure |
| `TelError` | teliox::Error | TEL processing failure |
| `DatabaseError` | String | ReDB failure (stringified) |
| `SigningError` | -- | Receipt signing failure |

### ApiError (HTTP wrapper, both witness + watcher)

Wraps `ActorError`, delegates to `ActorError::http_status_code()`.

---

## 6. DynamoDB Errors (`support/dynamodb`)

| Variant | When Produced |
|---------|---------------|
| `Sdk(msg)` | AWS SDK / DynamoDB API failure |
| `Serialization(msg)` | CESR/JSON/CBOR codec failure |
| `ConditionalCheckFailed(msg)` | DynamoDB condition expr failed (duplicate detection) |
| `NotFound(msg)` | Item not in table |
| `Generic(msg)` | Catch-all |

---

## 7. Cross-Crate Conversion Table

### Crypto -> Core

| From | To | Via |
|------|----|-----|
| `ed25519_dalek::SignatureError` | `KeysError::Ed25519DalekSignatureError` | `From` |
| `KeysError` | `keri_core::Error::SigningError` | `From` |
| `KeysError` | `prefix::error::Error::KeysError` | wrap |
| `cesrox::error::Error` | `prefix::error::Error::ParseError` | wrap |
| `prefix::error::Error` | `keri_core::Error::PrefixModuleError` | `From` |
| `SignatureError` | `keri_core::Error::KeyConfigError` | `From` |
| `SignatureError` | `VerificationError::SignatureError` | `#[from]` |
| `MoreInfoError` | `VerificationError::MoreInfo` | `#[from]` |
| `VerificationError` | `keri_core::Error::VerificationError` | `From` |

### Parse/Serialize -> Core

| From | To | Via |
|------|----|-----|
| `ParseError` | `keri_core::Error::DeserializeError` | `From` |
| `serializer_error::Error` | `keri_core::Error::SerdeSerError` | `From` |
| `said::error::Error` | `keri_core::Error::SAIError` | `From` |
| `VersionError` | `keri_core::Error::VersionError` | `From` |
| `ParseIntError` | `ThresholdError::ParseIntError` | `From` |

### Storage -> Core / TEL / Controller

| From | To | Via |
|------|----|-----|
| `RedbError` | `keri_core::Error` | `From` (feature `storage-redb`) |
| `RedbError` | `WitnessError::DatabaseError(String)` | manual `From` (stringifies) |
| `RedbError` | `ControllerError::RedbError` | `#[from]` |
| `RedbError` | `OobiRetrieveError::DbError` | `#[from]` |
| `redb::*Error` | `RedbError::*` | `#[from]` (4 variants) |
| `redb::*Error` | `teliox::Error::RedbError` | manual `From` (feature-gated) |
| `DynamoDbError` | `keri_core::Error::SemanticError` | `Into` |
| `DynamoDbError` | `teliox::Error::Generic` | `Into` |

### Core -> Higher-Level (all `#[from]`)

| From | To |
|------|----|
| `keri_core::Error` | `teliox::Error::KeriError`, `ControllerError::EventProcessingError`, `MechanicsError::EventProcessingError`, `WitnessError::KeriError` |
| `teliox::Error` | `ControllerError::TelError`, `WitnessError::TelError` |
| `ActorError` | `ApiError` (derive_more), `SendingError::ActorInternalError` |

### Controller Internal (all `#[from]`)

`SendingError` -> `ControllerError` / `MechanicsError` / `WatcherResponseError` / `BroadcastingError`. `MechanicsError` -> `ControllerError::Mechanic`. `WatcherResponseError` -> `ControllerError`. `ResponseProcessingError` / `BroadcastingError` / `OobiError` / `TransportError` -> `MechanicsError`.

---

## 8. Escrow Routing Decision Tree

### KEL: `BasicProcessor::basic_processing_strategy()`

| Error | Notification | Escrow | Result |
|-------|-------------|--------|--------|
| `EventOutOfOrderError` | `OutOfOrder` | `MaybeOutOfOrderEscrow` | `Ok(())` |
| `MissingDelegatorSealError` / `MissingDelegatingEventError` | `MissingDelegatingEvent` | `DelegationEscrow` | `Ok(())` |
| `EventDuplicateError` | `DupliciousEvent` | `DuplicitousEvents` | `Ok(())` |
| `NotEnoughSigsError` | `PartiallySigned` | `PartiallySignedEscrow` | `Ok(())` |
| `NotEnoughReceiptsError` | `PartiallyWitnessed` | `PartiallyWitnessedEscrow` | `Ok(())` |
| `MissingEvent` (receipt) | `ReceiptOutOfOrder` | receipt escrow | `Ok(())` |
| All others | -- | -- | `Err(..)` propagated |

### TEL: `TelEventProcessor::process()`

| Error | Notification | Escrow | Result |
|-------|-------------|--------|--------|
| `OutOfOrderError` | `TelNotification::OutOfOrder` | `OutOfOrderEscrow` | `Ok(())` |
| `MissingIssuerEventError` | `TelNotification::MissingIssuer` | `MissingIssuerEscrow` | `Ok(())` |
| `MissingRegistryError` | `TelNotification::MissingRegistry` | `MissingRegistryEscrow` | `Ok(())` |
| `EventAlreadySavedError` | -- | -- (idempotent) | `Ok(())` |
| All others | -- | -- | `Err(..)` propagated |

### Key Escrow Semantics

- **Escrow = not an error.** Escrowed events return `Ok(())` to caller. kerihost maps this to HTTP 202.
- **Cross-bus bridge:** TEL `MissingIssuerEscrow` implements `keri_core::Notifier` for `KeyEventAdded` -- KEL events trigger TEL escrow re-check.
- **SignatureVerificationError** in escrow: silently removes event (bad sig = permanent reject).
- **ReceiptVerificationError** in `PartiallyWitnessedEscrow`: bad receipt discarded, event stays escrowed.
- **`compute_state()`** swallows `EventOutOfOrderError` and `NotEnoughSigsError` (returns partial state).
