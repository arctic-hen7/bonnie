# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.3.2](https://github.com/arctic-hen7/bonnie/compare/v0.3.1...v0.3.2) (2021-09-05)


### Features

* ✨ added debug mode ([8891815](https://github.com/arctic-hen7/bonnie/commit/8891815313cd1a693e2ffecd2f63677528a598c8)), closes [#24](https://github.com/arctic-hen7/bonnie/issues/24)
* ✨ added self-documenting ability ([00417fd](https://github.com/arctic-hen7/bonnie/commit/00417fd782310bf86e40a6759a4d712bce88294c))
* **init:** ✨ allowed creating new config at custom path ([a81345b](https://github.com/arctic-hen7/bonnie/commit/a81345b43e569edb9d7c58eb5aef442221ef63c0))
* ✨ added support for default template ([#21](https://github.com/arctic-hen7/bonnie/issues/21)) ([7c3eec6](https://github.com/arctic-hen7/bonnie/commit/7c3eec6f3ae981bb64ea866bf7ebda2d29e7c956))


### Bug Fixes

* 🐛 fixed a few things from [#21](https://github.com/arctic-hen7/bonnie/issues/21) and cleaned up code ([dd4e273](https://github.com/arctic-hen7/bonnie/commit/dd4e2739fc892413b835d83bc786a36b2cce3562))


### Code Refactorings

* ♻️ removed unnecessary borrows ([f12c60e](https://github.com/arctic-hen7/bonnie/commit/f12c60e9ff85c5cad5d2144bb8ab3d7f10ececac))


### Documentation Changes

* **wiki:** 📝 updated wiki ([c4bd3cb](https://github.com/arctic-hen7/bonnie/commit/c4bd3cb2d873cef4fe1d0f6740eda49f8a820dab))

### [0.3.1](https://github.com/arctic-hen7/bonnie/compare/v0.3.0...v0.3.1) (2021-07-09)


### Bug Fixes

* 🐛 added custom delimiter support in shells and made default behaviour logical ([dd348ba](https://github.com/arctic-hen7/bonnie/commit/dd348ba4b22d07ca05f2938c868a7171650e957b)), closes [#17](https://github.com/arctic-hen7/bonnie/issues/17)


### Documentation Changes

* **wiki:** 📝 updated docs ([d910397](https://github.com/arctic-hen7/bonnie/commit/d910397ca83f91ad77ad603e5c0eaeca29300196))
* **wiki:** 📝 updated new issue link for templates ([1507fb7](https://github.com/arctic-hen7/bonnie/commit/1507fb72261f6ae7535646ed1ff3ea2dfa9bd838))
* **wiki:** 📝 updated wiki ([66be949](https://github.com/arctic-hen7/bonnie/commit/66be94933705f2789afa841dddafd3ca6f43987b))
* **wiki:** 📝 updated wiki ([f348de4](https://github.com/arctic-hen7/bonnie/commit/f348de40d956cabc6aa449e70a6e2e712db88f50))
* **wiki:** 📝 updated wiki ([a159aeb](https://github.com/arctic-hen7/bonnie/commit/a159aeb6d1aa1b4b6d852824ee24921d4d18343d))
* **wiki:** 📝 updated wiki ([2c6cc8d](https://github.com/arctic-hen7/bonnie/commit/2c6cc8d28e0370b147c9f889dd3f04519f8dc798))
* 📝 wrote the first iteration of the wiki ([be5f16c](https://github.com/arctic-hen7/bonnie/commit/be5f16c29b8d31b45320915ad3f0d4b6a606d6ef))

## [0.3.0](https://github.com/arctic-hen7/bonnie/compare/v0.2.1...v0.3.0) (2021-07-07)


### ⚠ BREAKING CHANGES

* changed default shell on Windows to `powershell`
* new invocation using flags for help/init/version commands
* significant error message changes

### Features

* ✨ added caching logic ([c9c4c4d](https://github.com/arctic-hen7/bonnie/commit/c9c4c4d3ee7917117af784b686cfd75201a50652)), closes [#14](https://github.com/arctic-hen7/bonnie/issues/14)
* ✨ added multistage commands, subcommands, shell/target control, and rewrote everything ([41209a3](https://github.com/arctic-hen7/bonnie/commit/41209a338b1a29357c418374b69f2ca5d5fbef65)), closes [#4](https://github.com/arctic-hen7/bonnie/issues/4) [#11](https://github.com/arctic-hen7/bonnie/issues/11) [#12](https://github.com/arctic-hen7/bonnie/issues/12)
* ✨ added new help/init/version commands ([8a8cb9c](https://github.com/arctic-hen7/bonnie/commit/8a8cb9c273a41206020cb3919ac0cb04d768d3f7))
* ✨ added support for custom cache file ([76fee05](https://github.com/arctic-hen7/bonnie/commit/76fee05ba0cc73d3bf65d3d63392fc51df56f79b))
* ✨ changed default shell on windows to powershell ([bdd2c67](https://github.com/arctic-hen7/bonnie/commit/bdd2c67b83d0417c46c24fd56c5e417ae8edfbc0)), closes [#15](https://github.com/arctic-hen7/bonnie/issues/15)


### Bug Fixes

* 🐛 fixed `%%` escaping ([7cd9810](https://github.com/arctic-hen7/bonnie/commit/7cd98101e0969f5fe2dd03bee1b1da3b0760888c))
* 🐛 fixed bones intersection operator ([49cabaf](https://github.com/arctic-hen7/bonnie/commit/49cabafd814b8ac60abf399f5efad79c4d09f1cc))
* 🐛 fixed bones parsing regex to allow variable initial whitespace ([fa5573f](https://github.com/arctic-hen7/bonnie/commit/fa5573faefcbb051db7fd57f5e39b86f8551cf98))
* 🐛 fixed vector ordering in schema ([200b3db](https://github.com/arctic-hen7/bonnie/commit/200b3db8d900f7a135553ce38ffeaf40f45a1185))
* 🐛 removed false positive warning for too many arguments ([b193e19](https://github.com/arctic-hen7/bonnie/commit/b193e19ed4dc370639669ac704d6d04dc91a3518))


### Code Refactorings

* ♻️ added ability to print warnings/info to buffer for testing ([910755d](https://github.com/arctic-hen7/bonnie/commit/910755db4afe8cc8964ad9d09da7ccef38e9a981))
* ♻️ cleaned up imports ([3a40616](https://github.com/arctic-hen7/bonnie/commit/3a40616dcf9650a18d33fec2e08c77d7bd2497a4))
* ♻️ removed unnecessary warning output extraction in argument interpolation ([be51340](https://github.com/arctic-hen7/bonnie/commit/be51340aa367fa671a5dd01293b5bd7bcaca520f))


### Documentation Changes

* 📝 added new help page ([cbbc806](https://github.com/arctic-hen7/bonnie/commit/cbbc8062b91d29edf322dc3594c658cd2c046662))
* 📝 added specification for experimental syntax ([7022670](https://github.com/arctic-hen7/bonnie/commit/7022670f8ff6a3e5c3e0463bc6de4dc6852629c2))
* 📝 added wiki submodule ([ce16314](https://github.com/arctic-hen7/bonnie/commit/ce163146c4c78a18a2cc6ad5d6e055c1703f3785))
* 📝 updated readme and wiki ([fbaac99](https://github.com/arctic-hen7/bonnie/commit/fbaac9923d3a3227705268284fd1e2b4e03b123d))

### [0.2.1](https://github.com/arctic-hen7/bonnie/compare/v0.2.0...v0.2.1) (2021-07-02)


### Bug Fixes

* ✅ fixed tests dependent on specific version ([490e1ff](https://github.com/arctic-hen7/bonnie/commit/490e1ff8b6c832b1d0139218728a84eabf0a9a2b))
* 🐛 fixed exit code mismanagement ([061b1c3](https://github.com/arctic-hen7/bonnie/commit/061b1c3352b6b5d850c4c383adafda4e5fe300eb)), closes [#13](https://github.com/arctic-hen7/bonnie/issues/13)

## [0.2.0](https://github.com/arctic-hen7/bonnie/compare/v0.1.3...v0.2.0) (2021-06-29)


### ⚠ BREAKING CHANGES

* mandated `version` key

### Features

* ✨ added musl install script ([9c8ecac](https://github.com/arctic-hen7/bonnie/commit/9c8ecac60d0a8bf6d48c7627ee2691a53b64c8db))
* ✨ added versioning system ([fbc4bc1](https://github.com/arctic-hen7/bonnie/commit/fbc4bc15f62726ce7c404778d1d1fa6fdc27f5e8))


### Bug Fixes

* 🚑 fixed incorrect version parse ordering ([8871dd6](https://github.com/arctic-hen7/bonnie/commit/8871dd64c26425019074e66c8451de937adbed1e))
* **install_scripts:** 🐛 fixed musl install script ([3f56f68](https://github.com/arctic-hen7/bonnie/commit/3f56f68ad8cc03855eedbc25d3e265057d91aa7a))


### Documentation Changes

* 📝 added gitter badge to readme ([#10](https://github.com/arctic-hen7/bonnie/issues/10)) ([b4aea0f](https://github.com/arctic-hen7/bonnie/commit/b4aea0f8a0452c5ff5e5320e4257e69d7ba64153))
* 📝 updated help page for version system ([dcfd598](https://github.com/arctic-hen7/bonnie/commit/dcfd5985f672122c12833756bb8c4246cc2aeb1f))
* 📝 updated readme for version system ([4a29232](https://github.com/arctic-hen7/bonnie/commit/4a29232b774da1eaf9026d22aa73ebe042244895))
* 📝 updated readme with Docker installation info ([1eaf1bf](https://github.com/arctic-hen7/bonnie/commit/1eaf1bf51649763ca79473ae217f8caabc2b04f8))

### [0.1.3](https://github.com/arctic-hen7/bonnie/compare/v0.1.2...v0.1.3) (2021-06-26)

### [0.1.2](https://github.com/arctic-hen7/bonnie/compare/v0.1.1...v0.1.2) (2021-06-25)


### Features

* ✨ added environment variable interpolation ([1af7f59](https://github.com/arctic-hen7/bonnie/commit/1af7f59758513c8ed092518466617a61d04bf46f))


### Bug Fixes

* 🚑 removed requriement for `env_files` key in config ([b1379e6](https://github.com/arctic-hen7/bonnie/commit/b1379e6685ca4e4746f17c69463f8eb80ff039d6))


### Code Refactorings

* 🔥 removed unnecessary `remove` function in command registry ([1d3cf1f](https://github.com/arctic-hen7/bonnie/commit/1d3cf1f37d94318d7323eb11e085f96b0286310c))


### Documentation Changes

* 📝 updated help page ([aeed110](https://github.com/arctic-hen7/bonnie/commit/aeed110d56a423b618f9728caed1cce1015f0180))
* 📝 updated readme ([9ba1bcc](https://github.com/arctic-hen7/bonnie/commit/9ba1bcc78c32f4150eb6857e18837d13af25128b))

### [0.1.1](https://github.com/arctic-hen7/bonnie/compare/v0.1.0...v0.1.1) (2021-05-02)


### Features

* 📝 added help page ([#1](https://github.com/arctic-hen7/bonnie/issues/1)) ([3a1a47d](https://github.com/arctic-hen7/bonnie/commit/3a1a47d230d33b105b6936804c2546d28be01f85))


### Documentation Changes

* 📝 added code of conduct ([7ca3efc](https://github.com/arctic-hen7/bonnie/commit/7ca3efc4dec71f3c3300e8324481f52c7a330247))
* 📝 added contributing guidelines ([38cb941](https://github.com/arctic-hen7/bonnie/commit/38cb94170c14d6962e6681c5888f5f0b6d64561b))
* 📝 added readme badges ([1391255](https://github.com/arctic-hen7/bonnie/commit/1391255384cac5d0abbee543a5b763c89bc905da))
* 📝 updated readme for releases page ([cb2c9fd](https://github.com/arctic-hen7/bonnie/commit/cb2c9fd5cadefb269e5ba1961f01a6a0dd72ce34))
