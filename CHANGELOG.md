# Changelog

## [0.3.3](https://github.com/eriandev/passnager/compare/v0.3.2...v0.3.3) (2026-10-03)

### 🐛 Bug Fixes

- **build:** correct clippy warning ([563d3eb](https://github.com/eriandev/passnager/commit/563d3eb76b107e8a686a346c77e704ecb522c8cc))
- **ci:** lint and typecheck in the release build job ([4abbc19](https://github.com/eriandev/passnager/commit/4abbc1953c295efaffae010ce5e90ca07db64a20))
- **ci:** stop the Windows release job from failing lint ([ef299eb](https://github.com/eriandev/passnager/commit/ef299ebfc5049ca8cda4002fcd244092f8eb2a13))
- **crypto:** reject a malformed nonce instead of panicking ([f40c484](https://github.com/eriandev/passnager/commit/f40c484f058c87c26b702bd7732234a291aef922))
- **db:** constrain the salt, nonce and wrapped-dek blobs in the schema ([22d5723](https://github.com/eriandev/passnager/commit/22d5723758579f5df3bd52ac6a065bf3db55a3c6))
- **db:** keep the category row shape camelCase like the other rows ([068c84a](https://github.com/eriandev/passnager/commit/068c84ac1e373f9b9dc5d95d9a1a56a5392a686d))
- **frontend:** do not inject a new entry into a filtered list ([bfa90bc](https://github.com/eriandev/passnager/commit/bfa90bc69f70593db28539bae3fda2b72a979bd0))
- **frontend:** enforce the same metadata length limits as the backend ([306dab2](https://github.com/eriandev/passnager/commit/306dab2eabede22a259ad3d8675a88dac5b9e683))
- **frontend:** give every button an explicit type ([ec323ac](https://github.com/eriandev/passnager/commit/ec323ac7c87472779f0632940d424d617df7adf9))
- **frontend:** label the controls the modals and lists leave unnamed ([6744bf8](https://github.com/eriandev/passnager/commit/6744bf80aef281e0ed7ff709a609c5a37f2938c6))
- **frontend:** omit rightIcon from the password input props ([f4873b0](https://github.com/eriandev/passnager/commit/f4873b00bc2b4c1cbb658716c2be47bb133f9cad))
- **frontend:** render a category that has no icon cleanly ([353d86f](https://github.com/eriandev/passnager/commit/353d86f9b0e6ab698d438fc656dd5d3b9ec39e91))
- **frontend:** stop reporting a delete that deleted nothing ([96fd382](https://github.com/eriandev/passnager/commit/96fd3821832ad7a22a434c02291d9e7f3567e2aa))
- **frontend:** submit the trimmed values the forms validate ([5480f5e](https://github.com/eriandev/passnager/commit/5480f5e74a64f12732771f33608f6e9dd7a0d47c))
- **frontend:** wait for the configuration check before redirecting ([1a5eaf0](https://github.com/eriandev/passnager/commit/1a5eaf09782964a2aac321e1ad5e3783dd736056))
- **master:** report a failed config check instead of claiming no master password ([0eae26d](https://github.com/eriandev/passnager/commit/0eae26d4ef867249dd32644ff102c754f52053d3))
- **security:** drop the opener plugin the app never calls ([336b333](https://github.com/eriandev/passnager/commit/336b33306c1d5a8f675fe4e65003eba6abc2d7ee))
- **security:** enforce the master password policy ([720a315](https://github.com/eriandev/passnager/commit/720a315be6b4a63f961493e3cba9964f94042a30))
- **security:** validate the vault metadata the schema leaves unbounded ([f77ecd5](https://github.com/eriandev/passnager/commit/f77ecd5ac3b7a6bb8840539b38d50dc27cc0f011))
- **style:** change selection color ([a7c59fa](https://github.com/eriandev/passnager/commit/a7c59fa09e0cf42515dfefdbfd135fcab5a9f6d0))

### 🚜 Code Refactoring

- **frontend:** hoist the category filter items into a derived ([86f9f77](https://github.com/eriandev/passnager/commit/86f9f7742dd7934374dd80074e86ae0b9a77aa47))
- **frontend:** use `null` as the single empty value for entry data ([93dcca1](https://github.com/eriandev/passnager/commit/93dcca1770d9a44855166e3be85f7f61062c9f54))

### 🎨 Styles

- **backend:** format code ([76ff53d](https://github.com/eriandev/passnager/commit/76ff53dcbdbd41da180953ccc581bd56406f4aad))

### Miscellaneous Tasks

- **build:** drop the unused scaffold assets and the duplicate stylesheet import ([5a1bd28](https://github.com/eriandev/passnager/commit/5a1bd28e81fab81e01903278f7e37e0a90360308))
- drop the unreachable and unreferenced code ([2834f84](https://github.com/eriandev/passnager/commit/2834f8463eaead313e1544d9483c48817c42ac1b))

## [0.3.2](https://github.com/eriandev/passnager/compare/v0.3.1...v0.3.2) (2026-09-30)

### 🐛 Bug Fixes

- **frontend:** force the dark toaster theme ([1283e05](https://github.com/eriandev/passnager/commit/1283e055178f65a9519e58cc293eaf13bcb70bb6))

### 🚜 Code Refactoring

- **ci:** read CHANGELOG notes with awk instead of Python ([5c3ff4c](https://github.com/eriandev/passnager/commit/5c3ff4c4c47f9d1d877eebf7c1712a82d236b01f))

### ⚙️ Continuous Integration

- bump actions/checkout from v4 to v7 (Node 24) ([4f691e0](https://github.com/eriandev/passnager/commit/4f691e04872c673ac9e64fe6361f39345cf3006d))

## [0.3.1](https://github.com/eriandev/passnager/compare/v0.3.0...v0.3.1) (2026-09-30)

### 🐛 Bug Fixes

- **ci:** parse CHANGELOG notes with bash and an explicit Python ([5f7008b](https://github.com/eriandev/passnager/commit/5f7008b84543f42fb5f0379ba958bd71fee4c99d))
- **frontend:** default the category icon and show it in every select ([82ba4ac](https://github.com/eriandev/passnager/commit/82ba4ac054765dd2180d9e0323766cadfc33c195))

### 🚜 Code Refactoring

- **frontend:** receive vault data and actions from parents ([7c2874d](https://github.com/eriandev/passnager/commit/7c2874d260b702d2476522aa766c227d23afaf7d))
- **frontend:** wire component props in app routes ([1aed683](https://github.com/eriandev/passnager/commit/1aed683ffeafb9d00be49f357c1f5f11ce9bd1bb))

## [0.3.0](https://github.com/eriandev/passnager/compare/v0.2.0...v0.3.0) (2026-09-30)

### ✨ Features

- **color:** validate colors and note body length in the schema ([65ba4e3](https://github.com/eriandev/passnager/commit/65ba4e36292c91603234507f86fd51564a7dc46d))
- **frontend:** replace remote favicons with a monospace initial avatar ([384beb4](https://github.com/eriandev/passnager/commit/384beb43718b44a05f9a1c6024cc9631685488eb))
- **frontend:** self-host fonts & drop Google Fonts ([9345960](https://github.com/eriandev/passnager/commit/93459606c8602c9d818cc6b0a34b4ac6b75043a9))
- **logging:** add a logger that splits console and file by build profile ([22f1621](https://github.com/eriandev/passnager/commit/22f1621992c649c85b9b64a22e9ce0c1c1d6096c))
- **notes:** implements encrypted notes ([8775e21](https://github.com/eriandev/passnager/commit/8775e210fe0ab09c0070834ceb71962ff7d1f6ff))
- **security:** add a strict CSP that allows no remote origins ([6f3e25d](https://github.com/eriandev/passnager/commit/6f3e25d5098de08133d2ff69d9f746f3c52f47ca))

### 🐛 Bug Fixes

- **security:** gate metadata-only note and password updates ([74ae89f](https://github.com/eriandev/passnager/commit/74ae89f339ebbdf016a0cbd09fac760ce1a6ddfe))
- **security:** route all vault access through the session ([c97e7ec](https://github.com/eriandev/passnager/commit/c97e7ec76c426bd0de91c2da9cd3e55f19f5828f))

### 📚 Documentation

- describe the new features, security & update preview ([dda4ca5](https://github.com/eriandev/passnager/commit/dda4ca5cc4211db2a7a45206bdcf083d945b865a))

### 📝 Tests

- **backend:** cover color, crypto, session and schema invariants ([cce5bce](https://github.com/eriandev/passnager/commit/cce5bcefcdfa108260f770bd8e266cd09653674f))

### 🚜 Code Refactoring

- **crypto:** make KDF cost injectable ([e7928d5](https://github.com/eriandev/passnager/commit/e7928d54dc90d5dcca3a9ec9b788073f27ea2ede))
- drop dead Settings type ([179da15](https://github.com/eriandev/passnager/commit/179da1531530c993d151db653f918aa12fa0010d))
- **master:** replace tuples with structs ([a9f4654](https://github.com/eriandev/passnager/commit/a9f4654bc0ccb5d1066d4b6615ea19abbe03b750))
- move color picker to component ([e243bfb](https://github.com/eriandev/passnager/commit/e243bfbcf992cc62ea20b44922a83681f39673b5))
- **notes:** decrypt on demand instead of caching contents ([0d9645c](https://github.com/eriandev/passnager/commit/0d9645cb1076c72147345575d558e8ea9ae26190))
- **startup:** add a startup module that aborts loudly and logs why ([b8912f0](https://github.com/eriandev/passnager/commit/b8912f06c329e9dd24eb4ef201312621526be172))
- use consts for default color values and the max number of chars for note content ([51d533d](https://github.com/eriandev/passnager/commit/51d533dafe6e4b008e55ff8b44cf9d0d33b91bce))

### ⚙️ Continuous Integration

- **release:** build from tags, gate on backend tests, and read notes from the changelog ([70b135b](https://github.com/eriandev/passnager/commit/70b135bb57d9efd5c219b2c7e4654bd3d3bc2d59))
- run lint, typecheck and backend tests on push and pull requests ([b243642](https://github.com/eriandev/passnager/commit/b243642e1c8e916b66f5aea3f9759483df270b6e))

### 🎨 Styles

- format code ([88364e6](https://github.com/eriandev/passnager/commit/88364e6945db9d0bad2a160783da384a1f966f19))

## [0.2.0](https://github.com/eriandev/passnager/compare/v0.1.1...v0.2.0) (2026-09-26)

### ✨ Features

- **cate:** add logic to handling categories ([d2a37fb](https://github.com/eriandev/passnager/commit/d2a37fbb0a006df7035d7b850917a12a1313eef9))
- **cate:** implements `category` page ([279011c](https://github.com/eriandev/passnager/commit/279011cf611240035ceb79b232539023813b2086))
- **security:** keep DEK server-side and zeroize key material ([c8ca078](https://github.com/eriandev/passnager/commit/c8ca0783a8bf1d38ba3ac8f6fab1d60313ec7e43))

### 🐛 Bug Fixes

- **ui:** correct modal width ([04ae698](https://github.com/eriandev/passnager/commit/04ae6981c9f801ad72b4969c646db4989d4dfee1))
- **ux:** run Argon2 commands off the main thread and disable submit while pending ([75858bd](https://github.com/eriandev/passnager/commit/75858bddf7748fb67769cf13cef5232e1c849197))

### 📚 Documentation

- update roadmap list ([2cfd938](https://github.com/eriandev/passnager/commit/2cfd93818d8115f496178fbcb264b702484e0c3a))

### 🚜 Code Refactoring

- separate card action & rename cards ([141bd06](https://github.com/eriandev/passnager/commit/141bd06b4d582c917a3cb7035a285b1267c61646))

### ⚙️ Continuous Integration

- fix release name ([6fcf829](https://github.com/eriandev/passnager/commit/6fcf829dbd95687ce4b413a0827c7395c9b9cf6e))

## [0.1.1](https://github.com/eriandev/passnager/compare/v0.1.0...v0.1.1) (2026-09-26)

### 🐛 Bug Fixes

- correct max width on `setup` page ([049139d](https://github.com/eriandev/passnager/commit/049139d275d631066af74afa4281edab0fbc1bc2))
- remove body vertical scroll ([57cbc5c](https://github.com/eriandev/passnager/commit/57cbc5c6b43538a7e0b65e6546cc05d3a30b83a0))
- remove components that don't exist ([d6304a0](https://github.com/eriandev/passnager/commit/d6304a056cda98577e2cffa02aa8db54a7aca610))
- update favicon ([d1a6bee](https://github.com/eriandev/passnager/commit/d1a6bee06c68eb567d277fdcb1f6cb8913c77578))

### 📚 Documentation

- add preview image ([4657308](https://github.com/eriandev/passnager/commit/465730888b6ab3ae90857750eedf8c63c4ced70b))

### ⚙️ Continuous Integration

- fix pnpm setup on action target Windows ([a93f315](https://github.com/eriandev/passnager/commit/a93f315855f527042168a4f2c4d8d0d3478956d7))

## 0.1.0 (2026-09-26)

### ✨ Features

- `auth` & `navigation` usables created ([3ce83a3](https://github.com/eriandev/passnager/commit/3ce83a38f7620d865783628b8861ee3d760c27f0))
- add `danger` button variant ([fbcda6a](https://github.com/eriandev/passnager/commit/fbcda6ae625b25dfb9b9175d9b6b735b59e4ee82))
- add `settings` page ([cdc5dad](https://github.com/eriandev/passnager/commit/cdc5dad7391330c47540c261b843626052773b3a))
- add `setup` & `unlock` pages ([5c70388](https://github.com/eriandev/passnager/commit/5c70388a1f490c1fc45be9dd5ba6de824e814488))
- add copy to clipboard logic ([f6cef78](https://github.com/eriandev/passnager/commit/f6cef789686c970c04ef42b31c5a040ffd50d075))
- add init db logic ([db53bb9](https://github.com/eriandev/passnager/commit/db53bb98fe048971bddd589bcfa83b1914fab70e))
- add logic to handling the master & passwords ([2cc84f1](https://github.com/eriandev/passnager/commit/2cc84f1c8f1cf1a0da0a25d2ad15ee516f15513a))
- add style & initial components ([e683fdb](https://github.com/eriandev/passnager/commit/e683fdbea9f3e3a24a7de905cdec45accc106913))
- **app:** update app config & icons ([21e3850](https://github.com/eriandev/passnager/commit/21e3850701d9c1426843e05b36085ec948524cff))
- implements `app` layout ([9c9f64b](https://github.com/eriandev/passnager/commit/9c9f64bd46ae669a64d648e02a2cb29501aec757))
- init template ([e9f424d](https://github.com/eriandev/passnager/commit/e9f424d13bc789bbb0256ba30d3e09d091f7b66d))
- **pass:** add logic to handle passwords ([cbd5a37](https://github.com/eriandev/passnager/commit/cbd5a372979b836e9928c09cfc81694d4a55496b))
- **pass:** add passwords list search ([017bb84](https://github.com/eriandev/passnager/commit/017bb84dd04c5ac38b331708fb7996643bafccf0))
- **pass:** implements the flows to add, edit & delete passwords ([9a71a43](https://github.com/eriandev/passnager/commit/9a71a4350b642c08fc740909862366dc5871fb7a))

### 📚 Documentation

- update `LICENSE` & `README` ([a9acfa6](https://github.com/eriandev/passnager/commit/a9acfa6646c9476b110e60b9320466ab18eacbcd))

### 🚜 Code Refactoring

- change sidebar lock icon on hover ([789db17](https://github.com/eriandev/passnager/commit/789db17c623c9fbe5e8cc9573d434834585d64d0))
- **db:** move `unix_timestamp` to db file ([8f07dd9](https://github.com/eriandev/passnager/commit/8f07dd90ea8437e9010cb728603eca11fe58b15d))
- **db:** rename the database response fields to camelCase ([42e1e50](https://github.com/eriandev/passnager/commit/42e1e50f851d9bad17ee27bc12bb421e76c064fb))
- remove `Label` component ([21f7d5a](https://github.com/eriandev/passnager/commit/21f7d5aa0710722cf31227fad70bdf8577c2f5e4))

### ⚙️ Continuous Integration

- update github action config & build scripts ([38b2e6e](https://github.com/eriandev/passnager/commit/38b2e6e962b4ead7cd115ed252f80e21e53994f8))

### Miscellaneous Tasks

- add action & scripts to update the changelog & semver ([974994c](https://github.com/eriandev/passnager/commit/974994cee43e2156e87c3c97d4e0f59132ec602b))
