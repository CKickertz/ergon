# Changelog

All notable changes to Aletheia are documented here.

---

## [0.10.1](https://github.com/CKickertz/ergon/compare/v0.10.0...v0.10.1) (2026-03-03)


### Features

* add hermeneus LLM provider crate (M1.5) ([8f99551](https://github.com/CKickertz/ergon/commit/8f99551e6962232a61d454c4d1f4aef9524582a7))
* add mneme SQLite session store crate (M1.1) ([38a4a35](https://github.com/CKickertz/ergon/commit/38a4a356a79e259d23c17fbcabfbb6a49c0d80ee))
* add nous agent pipeline crate (M2.1) ([f8ec172](https://github.com/CKickertz/ergon/commit/f8ec1729a16776335714aa3e045621bde77b023d))
* add oikos instance scaffold and migration script (M0a.1) ([3975ef6](https://github.com/CKickertz/ergon/commit/3975ef63df1a1b2e25b0a882ea2f7f0be3736433))
* add pathGuard config to disable filesystem containment ([175683e](https://github.com/CKickertz/ergon/commit/175683ec9eef2f175da8b3dcf4757bd73d6280c6))
* **agora:** channel registry and Signal provider ([ef0cce9](https://github.com/CKickertz/ergon/commit/ef0cce9ffd525058fa88edb60025b500b0432dbd))
* **agora:** inbound message routing with binding resolution ([2eb6ceb](https://github.com/CKickertz/ergon/commit/2eb6cebd9fe4b4e35a7f490554383d8470cf830e))
* **agora:** inbound message routing with binding resolution ([#421](https://github.com/CKickertz/ergon/issues/421)) ([2eb6ceb](https://github.com/CKickertz/ergon/commit/2eb6cebd9fe4b4e35a7f490554383d8470cf830e))
* **agora:** multi-platform channel abstraction — Phases 1-6 ([f27af19](https://github.com/CKickertz/ergon/commit/f27af190b50d19397e319aa39b5cc58de53be868))
* **agora:** Signal inbound listener and message routing ([4ea6039](https://github.com/CKickertz/ergon/commit/4ea603902fcab222a0f8eddf83fc92eca2afcd86))
* **aletheia:** binary entrypoint with Clap CLI ([fc20d11](https://github.com/CKickertz/ergon/commit/fc20d11882eb3713ae9ca325d60fe02f37724b07))
* **auth:** JWT, RBAC, sessions, audit, retention, and TLS ([fd58a62](https://github.com/CKickertz/ergon/commit/fd58a62c87453ec4dce5691d47f6c024b8417665))
* CozoDB validation gate passes all 5 tests (M1.2a) ([c970e38](https://github.com/CKickertz/ergon/commit/c970e38669e1a3a871ca1940ec5306b57ea92252))
* cross-crate integration tests + vendored CozoDB patches ([#362](https://github.com/CKickertz/ergon/issues/362)) ([ee1c935](https://github.com/CKickertz/ergon/commit/ee1c9350017214aadbaa46664f9f3d5c2896c042))
* **dianoia:** collaborative API, task system, planning workspace ([39f0fda](https://github.com/CKickertz/ergon/commit/39f0fda46e747589c6cb79ce00569dbf81457f47))
* **dianoia:** Dianoia v1 — persistent multi-phase planning runtime ([ad688a3](https://github.com/CKickertz/ergon/commit/ad688a364ffbb43d9e6c0357685403346cb9f66a))
* **dianoia:** enhanced execution engine and planning UI ([61754c5](https://github.com/CKickertz/ergon/commit/61754c56d2d20fc0e324037f8e747d5399b7a9fd))
* **dianoia:** execution engine, v1 completion, and context wiring ([7312d4d](https://github.com/CKickertz/ergon/commit/7312d4d750b9d0dfb5c612b1ce9a746a4bb40ccd))
* **dianoia:** execution resilience — stuck detection, completion assertions, iteration caps ([0c96e1c](https://github.com/CKickertz/ergon/commit/0c96e1cde53dc83c572ef8937dd323b292e12b87))
* **dianoia:** orchestrator context assembly (Spec 32 Phase 1b) ([97df0c7](https://github.com/CKickertz/ergon/commit/97df0c75619a83850f55b023643771e0184b5c85))
* **dianoia:** v1.1 standards and GSD phases 1-3 ([891e8d1](https://github.com/CKickertz/ergon/commit/891e8d167a3dcfc9cb8acb7c76712b34881e95c9))
* **dianoia:** v2 file-backed state and context packet ([ac5c7e9](https://github.com/CKickertz/ergon/commit/ac5c7e98cc1f4eb4309375b3d1070ae520018d24))
* **hermeneus:** Anthropic Messages API provider — streaming SSE, retry with backoff, thinking + tool_use support ([bb5b324](https://github.com/CKickertz/ergon/commit/bb5b324a22852ff917b68a94549a293603993e95))
* **hermeneus:** model fallback + model ID reference ([#400](https://github.com/CKickertz/ergon/issues/400)) ([5685d40](https://github.com/CKickertz/ergon/commit/5685d40ef5bd51c5327fa9922648283f4090200f))
* **hermeneus:** model fallback on streaming retry exhaustion ([5685d40](https://github.com/CKickertz/ergon/commit/5685d40ef5bd51c5327fa9922648283f4090200f))
* **hermeneus:** OAuth auto-refresh and pylon retry queue ([d6e6b75](https://github.com/CKickertz/ergon/commit/d6e6b75f39688ca13c6ff891612e536b21d9ba8b))
* koina + taxis Rust crates (M0b) ([#358](https://github.com/CKickertz/ergon/issues/358)) ([85cd60f](https://github.com/CKickertz/ergon/commit/85cd60fcd7e29925c4f292e7e4867e56feceeeac))
* **melete:** context distillation engine ([dae5ad2](https://github.com/CKickertz/ergon/commit/dae5ad2b469f8daa481f72fe62e41abdb9e590e7))
* **memory:** emergency distillation, memory health, and audit CLI ([ec2fcdb](https://github.com/CKickertz/ergon/commit/ec2fcdb1fa5e0f3753e9090a1f480b08ec17913b))
* **memory:** extraction pipeline and contradiction detection ([6679f28](https://github.com/CKickertz/ergon/commit/6679f28e0df8ff0ebf1314d678de6c1fd0a671d9))
* **memory:** memory pipeline and PII detection ([9879257](https://github.com/CKickertz/ergon/commit/9879257ce611bdd8bcfda9c6793d53ca4138a4e9))
* **memory:** reinforcement loop, noise filter, and domain re-ranking ([504242d](https://github.com/CKickertz/ergon/commit/504242d81ea97a26a9d4c37eafa42395ad9d6aa8))
* **memory:** self-configuring LLM backend and webchat UX ([c71e5c3](https://github.com/CKickertz/ergon/commit/c71e5c3fed2f1644a3bea56c1e5eb8c14c554348))
* **memory:** tech debt cleanup, RELATES_TO baseline, and flush tracking ([19b6889](https://github.com/CKickertz/ergon/commit/19b688920bd490b72c87ec8e4a0af944914a3616))
* **memory:** Voyage-4 embeddings migration and UI redesign ([6a7a7ae](https://github.com/CKickertz/ergon/commit/6a7a7aed4f6bac66fae55de40958b43a9e60cf1d))
* migrate runtime paths to oikos instance structure ([#355](https://github.com/CKickertz/ergon/issues/355)) ([0c4e3c8](https://github.com/CKickertz/ergon/commit/0c4e3c8cdbf5cf4f76807341ff64f9f7e7d56e26))
* migrate runtime paths to oikos instance structure ([#359](https://github.com/CKickertz/ergon/issues/359)) ([82527f3](https://github.com/CKickertz/ergon/commit/82527f3b83218d33e8ea480e901c75782ed5a71a))
* **mneme-engine:** scaffold, safety, and knowledge store integration (phases 1-3) ([014aa41](https://github.com/CKickertz/ergon/commit/014aa4151fb4960e96a82f91292376214e434f40))
* **mneme-engine:** v1.0 CozoDB absorption — hybrid retrieval + idiom migration ([fa6b17d](https://github.com/CKickertz/ergon/commit/fa6b17db4a9350ca7845b105abdc7e1701c5bdca))
* **mneme:** 6-factor recall scoring engine (M1.4) ([ea3d44d](https://github.com/CKickertz/ergon/commit/ea3d44d7c3460f6e743771dbe8ba53cc5f254e4b))
* **mneme:** add knowledge types and feature-gated storage backends (M1.2) ([6dd48d7](https://github.com/CKickertz/ergon/commit/6dd48d7ede06219f00a8a9681538cd41f7956df6))
* **mneme:** CozoDB knowledge store schema and query templates (M1.2) ([3ae23ca](https://github.com/CKickertz/ergon/commit/3ae23ca0d1c782bdcf8c1c6506520f0e3505f620))
* **mneme:** embedding provider trait with mock backend (M1.3) ([5d07fa1](https://github.com/CKickertz/ergon/commit/5d07fa1652f7df6fe1dc040a8bcd42bb2e346920))
* **mneme:** fastembed-rs embedding provider ([6ffcce7](https://github.com/CKickertz/ergon/commit/6ffcce7b7d44f70e508aeecaf8b6752d4cb4a8b1))
* **mneme:** Mem0 integration, Neo4j migration, and fork runtime ([4c9bfd4](https://github.com/CKickertz/ergon/commit/4c9bfd45f469a6a515c842359d7591d97d966b52))
* **mneme:** memory evolution, deliberation, and discovery engine ([b655e73](https://github.com/CKickertz/ergon/commit/b655e73bc158532d2a2217d9fa66391ce6d97b1a))
* **mneme:** temporal decay, BM25 hybrid search, and invariant tests ([63db2ad](https://github.com/CKickertz/ergon/commit/63db2ad0c4ea1a63fa1bc5b8a7611c2964dd4c60))
* **mneme:** typed Datalog query builder — compile-time schema validation ([442ec03](https://github.com/CKickertz/ergon/commit/442ec03e881d79d61d7283052e59d8b20a3d0e0d))
* **mneme:** working state, release workflow, and error boundaries ([390d72f](https://github.com/CKickertz/ergon/commit/390d72f267f2da83ada2b8de9440097ddeb70fe3))
* **nous:** consolidate architecture, knowledge graph with ontology ([289f7c3](https://github.com/CKickertz/ergon/commit/289f7c379b03f25a2ca1f17a0b828cfda3c14cd0))
* **nous:** context bootstrap — oikos cascade assembly, token budget, tool tiers ([71970a0](https://github.com/CKickertz/ergon/commit/71970a0e3a9705c6db89405fc5358a8dd0ee4a60))
* **nous:** distillation improvements and chunked extraction ([179eea0](https://github.com/CKickertz/ergon/commit/179eea0c39c34479e48a1f70d52c0c3a7e40279a))
* **nous:** distillation, structured logging, and tool governance ([8858917](https://github.com/CKickertz/ergon/commit/8858917be02520bbd4230ed518112ecc1f16fc26))
* **nous:** execute stage with LLM call and tool iteration loop ([5e62822](https://github.com/CKickertz/ergon/commit/5e628221e86bc8b1b778247c0e91ba52f23a9c3a))
* **nous:** finalize pipeline stage — persist messages and track usage ([bb13d22](https://github.com/CKickertz/ergon/commit/bb13d22c3872744d5e422d27db6f6eef616be3b0))
* **nous:** finalize pipeline stage — persist messages and track usage ([#420](https://github.com/CKickertz/ergon/issues/420)) ([bb13d22](https://github.com/CKickertz/ergon/commit/bb13d22c3872744d5e422d27db6f6eef616be3b0))
* **nous:** history pipeline stage — load conversation context ([fae67d9](https://github.com/CKickertz/ergon/commit/fae67d90bb2a2cdea330a6ec54c72825699e52a1))
* **nous:** history pipeline stage — load conversation context ([#419](https://github.com/CKickertz/ergon/issues/419)) ([fae67d9](https://github.com/CKickertz/ergon/commit/fae67d90bb2a2cdea330a6ec54c72825699e52a1))
* **nous:** history pipeline stage — load conversation context ([#426](https://github.com/CKickertz/ergon/issues/426)) ([8c10231](https://github.com/CKickertz/ergon/commit/8c1023107ed47972534a0b992b86c52bfb966c39))
* **nous:** hook system, agent export, spec completion, and onboarding ([bdc12b9](https://github.com/CKickertz/ergon/commit/bdc12b98613feec073022515101982c513aa8d4c))
* **nous:** hot-reload, plan mode, and graph intelligence ([e269b67](https://github.com/CKickertz/ergon/commit/e269b675523f6188ac75c83445f1d556b2eba91f))
* **nous:** iterative recall with terminology discovery + tool repetition detection ([c3126cf](https://github.com/CKickertz/ergon/commit/c3126cfbf69445bc871e1f04cc36dcdc1a55fa94))
* **nous:** KnowledgeVectorSearch bridge for recall pipeline ([522b4e6](https://github.com/CKickertz/ergon/commit/522b4e618d388865c634652fefa7ff6bcc7058de))
* **nous:** message queue, sub-agent spawn, and session auth ([d89abeb](https://github.com/CKickertz/ergon/commit/d89abeb457f9ad87f64cd58f53337c29d16c22d6))
* **nous:** NousActor tokio actor with inbox, lifecycle, and message dispatch ([a410aff](https://github.com/CKickertz/ergon/commit/a410affcc23e4f6058b0c0b4517fde0131565910))
* **nous:** OAuth auth, group messaging, and history replay ([b15258b](https://github.com/CKickertz/ergon/commit/b15258b91a54baca57e3401ae131d8851bc11161))
* **nous:** parallel tools, convergence, and batch dispatch ([0bd53dd](https://github.com/CKickertz/ergon/commit/0bd53ddf96b4f120d78d484a68c7a4341a6e62ca))
* **nous:** prosoche attention daemon and evaluation framework ([aeeb700](https://github.com/CKickertz/ergon/commit/aeeb7007f860758315cb851dd610d4106a72ee04))
* **nous:** quick-win specs, dead code cleanup, and code patching ([29baf02](https://github.com/CKickertz/ergon/commit/29baf02a67c71f21963b1b118a92872512d5dbf2))
* **nous:** recall pipeline stage with knowledge retrieval ([cc8cb85](https://github.com/CKickertz/ergon/commit/cc8cb85bb9204d755edf0f1de76055e167831b33))
* **nous:** research stack, service watchdog, and workspace tooling ([fb97230](https://github.com/CKickertz/ergon/commit/fb97230f01ff1635174d809f94b5b9c7eb5baa63))
* **nous:** routing, planning tools, MCP server, and CI/CD ([7247943](https://github.com/CKickertz/ergon/commit/7247943c67dc32cb86eb496e4eabae606c7e1df0))
* **nous:** runtime v2 clean-room rewrite ([fd9b2ad](https://github.com/CKickertz/ergon/commit/fd9b2ad2d845ac4c380a8d40e10812329d7da610))
* **nous:** search, embeddings, metrics, and tool policy ([d7ae48e](https://github.com/CKickertz/ergon/commit/d7ae48e5f6ac654634c8ecc5746dd5b47a52ae6b))
* **nous:** slash commands, thinking mode, and stop button ([54ac1f4](https://github.com/CKickertz/ergon/commit/54ac1f4b271de7b846fe600413674ed7caf3b089))
* **nous:** Spec 25 IDE, policy documents, and per-agent params ([42a66af](https://github.com/CKickertz/ergon/commit/42a66af6e3e8769e09303e5e05c6800699e23c08))
* **nous:** sub-agent infrastructure and session continuity ([550172b](https://github.com/CKickertz/ergon/commit/550172bb56d8ad1a2f27f160ffb124b6c6931dad))
* **nous:** thinking mode, distillation phases, and context editing ([c9f9990](https://github.com/CKickertz/ergon/commit/c9f99904c048635ec63e57fdfeefe7bd598e8fab))
* **nous:** TTS pipeline, event bus, and skill learning ([8124e08](https://github.com/CKickertz/ergon/commit/8124e08f6b22a6aa9296dbd657dfca1804c3a17e))
* **nous:** turn safety and error propagation ([9d88def](https://github.com/CKickertz/ergon/commit/9d88def567f03b2583b9caaff8861ff7c0cb8545))
* **nous:** unified thread model and data privacy module ([eef5789](https://github.com/CKickertz/ergon/commit/eef578917c978aa9e2a5fa5dae425bdf76884687))
* **nous:** zero-config onboarding, hermeneus OAuth, credential pill ([4ae5bae](https://github.com/CKickertz/ergon/commit/4ae5baeac569d380a126bd6bc29f044b9f1665a5))
* **organon:** gitignore-aware workspace indexer with live watcher ([dca103e](https://github.com/CKickertz/ergon/commit/dca103e46036934e81ce3d55b02833868d410665))
* **organon:** implement workspace tool executors ([2a672db](https://github.com/CKickertz/ergon/commit/2a672db7adc6cd784df60aa33b32e0daf7d29098))
* **organon:** tool registry + definition types — ToolDef, InputSchema, ToolExecutor trait, built-in stubs ([3178433](https://github.com/CKickertz/ergon/commit/317843386dee076e90c03a6fd4456f5d5622e1eb))
* **pylon:** Axum HTTP gateway — session CRUD, SSE streaming, health check ([31d9f5e](https://github.com/CKickertz/ergon/commit/31d9f5e4e01eb1c964b8f5ca3c763073e10a7a1b))
* **pylon:** JWT auth middleware via symbolon ([2f47df5](https://github.com/CKickertz/ergon/commit/2f47df5839590ecc56da5f44ddec0f4b19de1d19))
* **pylon:** route message handling through NousActor ([d7bac13](https://github.com/CKickertz/ergon/commit/d7bac137a36518585b0d5335ddd5d31d7c5141ae))
* **recall:** wire embedding provider and scaffold recall pipeline ([9dde7e4](https://github.com/CKickertz/ergon/commit/9dde7e4e9eb06b9b46ab474aa32832bf45ab6bc9))
* **symbolon:** JWT sessions, API keys, argon2 passwords, RBAC ([a659ede](https://github.com/CKickertz/ergon/commit/a659ede4985be884a412d19aefe3aaa4ec430bd6))
* **taxis:** auto-migrate legacy ~/.aletheia/ state files ([#402](https://github.com/CKickertz/ergon/issues/402)) ([fbb40d5](https://github.com/CKickertz/ergon/commit/fbb40d582aa8a7ca8e3a4e9488df74a5755e7a6f))
* **taxis:** auto-migrate legacy ~/.aletheia/ state files on startup ([fbb40d5](https://github.com/CKickertz/ergon/commit/fbb40d582aa8a7ca8e3a4e9488df74a5755e7a6f))
* **taxis:** figment-based config loading with YAML cascade ([d57da70](https://github.com/CKickertz/ergon/commit/d57da70d619b3f6b72cb1b5fb68381bde67e786a))
* **taxis:** oikos cascade resolution (M0a phases 0a.2–0a.4) ([#356](https://github.com/CKickertz/ergon/issues/356)) ([81ae32b](https://github.com/CKickertz/ergon/commit/81ae32b5ee000227c593973ccaff0c039c3b3ed7))
* **thesauros:** add domain pack loader for external knowledge injection ([#2](https://github.com/CKickertz/ergon/issues/2)) ([f6568b7](https://github.com/CKickertz/ergon/commit/f6568b76c432aebf2dd18a45c128241ccf55fea6))
* **thesauros:** add domain tagging and config overlay merging ([#4](https://github.com/CKickertz/ergon/issues/4)) ([dc5eb6a](https://github.com/CKickertz/ergon/commit/dc5eb6a0ef9234091c6324e3f710c6db5973e051))
* **thesauros:** register domain pack tools at startup ([#3](https://github.com/CKickertz/ergon/issues/3)) ([5647c39](https://github.com/CKickertz/ergon/commit/5647c39e17c52a84eb06d5273d6a0da4ffc50b3b))
* **tui:** TUI dashboard MVP — Phase 1-3 complete ([c679a94](https://github.com/CKickertz/ergon/commit/c679a9496119192d184873712533c0aa2dd61f2c))
* **ui:** CodeMirror highlighting and TreeContextMenu ([0ecceb9](https://github.com/CKickertz/ergon/commit/0ecceb936543d2ef5217fe817df5fc77e491ffcf))
* **ui:** credential management and onboarding wizard ([ff8e37a](https://github.com/CKickertz/ergon/commit/ff8e37a8b90c5de34612f01315e176c2301173d2))
* **ui:** gold theme, design system, accessibility, and onboarding ([a7e3e49](https://github.com/CKickertz/ergon/commit/a7e3e49e1f0590dc2cddba71ab51b28aaf429def))
* **ui:** graph visualization, file explorer, and pipeline decomposition ([57c55d7](https://github.com/CKickertz/ergon/commit/57c55d7e9e67f54971c55c9a47d927281467bb6d))
* **ui:** onboarding system, mobile fixes, and SSE reliability ([350e1d2](https://github.com/CKickertz/ergon/commit/350e1d20c1f1749cba44358a79ace41db7883b03))
* **ui:** SSE reliability, memory client, and graph palette ([9d3a57e](https://github.com/CKickertz/ergon/commit/9d3a57e88225477f1d4c4609282140b20a0671ca))
* **ui:** Svelte 5 web UI with streaming backend ([1d6ee5c](https://github.com/CKickertz/ergon/commit/1d6ee5c3cadae626de38f18f65551a733e50d8da))
* **ui:** syntax highlighting, sidebar, slash commands, file uploads ([5c00960](https://github.com/CKickertz/ergon/commit/5c00960a091a60f20bc7700e7dad3b0d2f946029))
* **ui:** UI overhaul — light theme, horizontal agent bar, debounce ([204e5ee](https://github.com/CKickertz/ergon/commit/204e5eee9761af3661c9fc6c86f2bb057fbe4b76))
* **v1.2:** onboarding, Mac support, and workspace cleanup ([6e79cdd](https://github.com/CKickertz/ergon/commit/6e79cddb5744f4e70d4a8b8cf4ffd670aa5bca6e))
* **v1.4:** workspace architecture and anchor paths ([0fda749](https://github.com/CKickertz/ergon/commit/0fda749d2fd3825f8cf4f79f73a2df44c2a47584))


### Bug Fixes

* align with coding standards — static assertions, serde tests, tracing ([be87e2a](https://github.com/CKickertz/ergon/commit/be87e2ab0ea8963fa8549c10df14c67da2deb52e))
* deduplicate registries and reconcile binary with actor AppState ([573fd0e](https://github.com/CKickertz/ergon/commit/573fd0ed599fc5e050902902bcaeac4b3716bf9d))
* **hermeneus:** convert mid-stream APIError to ProviderError for retry ([#394](https://github.com/CKickertz/ergon/issues/394)) ([b1b673e](https://github.com/CKickertz/ergon/commit/b1b673e0cb81acbc74eb942655d864ccd4a262f8))
* **hermeneus:** resilient retry for API overload errors ([#397](https://github.com/CKickertz/ergon/issues/397)) ([cd2796d](https://github.com/CKickertz/ergon/commit/cd2796d77ac54b39c7033acecc59a25cd0298915))
* **hermeneus:** Rust streaming retry + SSE error typing ([#403](https://github.com/CKickertz/ergon/issues/403)) ([be62e8b](https://github.com/CKickertz/ergon/commit/be62e8b249bfcf053d72e2f534d6a3ce3f3ff83f)), closes [#398](https://github.com/CKickertz/ergon/issues/398)
* **integration-tests:** update e2e tests for Arc wrappers in AppState ([e076864](https://github.com/CKickertz/ergon/commit/e0768640a53fe85c43c24eef7606d05c95261a06))
* **mneme-engine:** clean all clippy warnings — zero warnings across workspace ([5ba7d9a](https://github.com/CKickertz/ergon/commit/5ba7d9aebee7766cfcfd51a4c2f89a955286d74e))
* **mneme:** graph score aggregation and RRF rank encoding ([58a709b](https://github.com/CKickertz/ergon/commit/58a709b2ee5b71a7d398e395d8a329f5e61e0feb))
* **mneme:** hybrid search bugs — graph score aggregation and RRF encoding ([#422](https://github.com/CKickertz/ergon/issues/422)) ([58a709b](https://github.com/CKickertz/ergon/commit/58a709b2ee5b71a7d398e395d8a329f5e61e0feb))
* **mneme:** hybrid search correctness bugs (BUG-01..03) ([d407855](https://github.com/CKickertz/ergon/commit/d4078557dbae3d97a869c8f30c783a192fdd1f60))
* **nous:** runtime hardening for plugins, models, and errors ([5bc6687](https://github.com/CKickertz/ergon/commit/5bc6687805df3f4a4b245e9efd9b6ed7a350245e))
* **qa:** Rust codebase audit — 3 fixes, 2 bugs documented ([#361](https://github.com/CKickertz/ergon/issues/361)) ([6e72066](https://github.com/CKickertz/ergon/commit/6e720667811e8f5d10f50dbb584b571de81a4e0c))
* remove unsound serde_yml dependency (Dependabot [#18](https://github.com/CKickertz/ergon/issues/18), [#19](https://github.com/CKickertz/ergon/issues/19)) ([0f62afa](https://github.com/CKickertz/ergon/commit/0f62afae3582361173a9c83e98eba9fb1626ab65))
* **security:** resolve CodeQL scanning alerts ([#388](https://github.com/CKickertz/ergon/issues/388)) ([b7f37e8](https://github.com/CKickertz/ergon/commit/b7f37e88ce09d6d04f03c16ec0912a63675a6840))
* **security:** resolve CodeQL scanning alerts across infrastructure ([b7f37e8](https://github.com/CKickertz/ergon/commit/b7f37e88ce09d6d04f03c16ec0912a63675a6840))
* **taxis:** migrate remaining ~/.aletheia paths to oikos instance — closes [#372](https://github.com/CKickertz/ergon/issues/372) ([9af5920](https://github.com/CKickertz/ergon/commit/9af59207831e560f486250bcf6f0fe249ea4e84b))
* **taxis:** migrate remaining ~/.aletheia paths to oikos instance ([#373](https://github.com/CKickertz/ergon/issues/373)) ([9af5920](https://github.com/CKickertz/ergon/commit/9af59207831e560f486250bcf6f0fe249ea4e84b))
* **ui:** SSE reliability, ChatView import, and UI fixes ([4725e06](https://github.com/CKickertz/ergon/commit/4725e06ea937f59877d9d6c1c55649a3a75af097))

## [0.10.6] - 2026-02-22

### Fixed
- **Pipeline error `turn_complete`** — Streaming pipeline errors now yield a proper `turn_complete` event with error outcome, so clients compose correct assistant messages instead of saving partial text without attribution
- **SSE stream error handler** — Guarded `controller.enqueue` in the stream endpoint's outer catch to prevent uncaught exception when client has already disconnected
- **Active turns counter floor** — Global `activeTurns` counter uses `Math.max(0, ...)` to prevent negative values
- **SSE connection resilience** — Heartbeat timer (45s) detects dead connections, reconnect clears stale `activeTurns`, disconnect fallback polls every 5s, fetch stream has 2-min read timeout
- **Connection listener accumulation** — `initConnection()` now cleans up previous listener before adding a new one, preventing duplicate status callbacks on re-initialization
- **App.svelte effect cleanup** — `$effect` returns `disconnect()` cleanup to close EventSource when effect re-runs

---

## [0.10.5] - 2026-02-22

### Added
- **Web UI agent creation** (Spec 5 P5) — "+" button in sidebar opens inline form to create agents. Calls `POST /api/agents` which scaffolds workspace, updates config, and hot-reloads the runtime. New agent is auto-selected for immediate onboarding.
- **`aletheia init` wizard** (Spec 5 P6) — First-run setup: prompts for Anthropic API key, gateway port, and first agent. Writes credentials, config, and scaffolds workspace in one flow.

### Specs Completed
- **Spec 5** Plug-and-Play Onboarding — 6/6 phases

---

## [0.10.4] - 2026-02-22

### Added
- **`aletheia agent create`** (Spec 5 P1-P3) — CLI command scaffolds a new agent workspace from `_example/` template with onboarding SOUL.md. Supports `--id`, `--name`, `--emoji` flags or interactive prompts. Automatically updates `aletheia.json` with agent entry and web binding.
- **Onboarding SOUL.md** (Spec 5 P4) — Scaffolded agents get an onboarding prompt as their initial SOUL.md. The agent interviews its operator to learn name, domain, working style, and boundaries, then writes its own `SOUL.md`, `USER.md`, and `MEMORY.md`. Zero runtime changes — the agent naturally transitions out of onboarding by overwriting SOUL.md via the write tool.

---

## [0.10.3] - 2026-02-22

### Added
- **Runtime code patching** (Spec 26 P5) — `propose_patch` and `rollback_patch` tools let agents modify their own source code with automated safety gates (tsc + vitest + backup/restore). Patchable dirs: `organon/`, `nous/`, `distillation/`, `daemon/`. Rate limited: 1/hr/agent, 3/day total.
- **Evolutionary config search** (Spec 26 P6) — `evolution:nightly` cron mutates pipeline configs (recall, tools, notes) via Haiku, benchmarks variants against approval-signal tasks, promotes winners with 24h auto-adopt window and Signal notification. Archive capped at 5 variants per agent.
- **Checkpoint time-travel** (Spec 21 P4) — `aletheia fork <session-id> --at <N>` creates a new session branched from distillation checkpoint N. API endpoints: `GET /api/sessions/:id/checkpoints`, `POST /api/sessions/:id/fork`.
- **Pipeline config save** — `savePipelineConfig()` function for writing validated configs back to disk

### Specs Completed
- **Spec 26** Recursive Self-Improvement — 6/6 phases
- **Spec 21** Agent Portability — 4/4 phases

---

## [0.10.2] - 2026-02-21

### Added
- **Update notification badge** (Spec 3 P3b) — TopBar polls `/api/system/update-status`, shows green version badge when update available
- **Update API endpoint** (Spec 3 P3c) — `POST /api/system/update` runs git pull + rebuild, returns status
- **Credential failover** (Spec 3 P4a) — `ProviderRouter.complete()` tries backup API keys from `~/.aletheia/credentials/anthropic.json` on recoverable errors (429/5xx)

### Fixed
- **`[object Object]` in webchat** — LLM sometimes returns structured objects in string arrays during distillation extraction and working state updates. Added `toStringArray()` filter to all 9 array fields across `extract.ts` and `working-state.ts`.

### Changed
- **Tool workspace scope removed** — file tools (read, write, edit, ls, grep, find) no longer constrained to workspace + allowedRoots via `safePath()`. Now use plain `resolve()`, matching `exec` tool behavior. Deleted `safe-path.ts` and tests.

### Specs Completed
- **Spec 3** Auth & Updates — all phases complete

---

## [0.10.1] - 2026-02-21

### Added
- **Plugin auto-discovery** (Spec 18 P4) — scans `shared/plugins/` for plugin directories, merges with explicitly configured paths, `aletheia plugins list` CLI command
- **Plugin path safety** (Spec 18 P5) — `realpathSync` traversal guard on auto-discovered plugins, symlink escape prevention. Explicit config paths bypass validation.
- **Loop guard hook template** (Spec 18 P6) — `shared/hooks/_templates/loop-guard.yaml` + `.sh`, detects stuck tool-call patterns via sentinel file
- **Encrypted memory init** (Spec 20 P4) — AES-256-GCM encryption wired at startup with salt persistence. `encryptIfEnabled`/`decryptIfNeeded` already in store; this adds `initEncryption()` call and salt management.
- **Agent file import** (Spec 21 P2) — `aletheia import <file.agent.json>` restores workspace files, sessions with ID remapping, messages, notes, working state, distillation priming
- **Scheduled backups** (Spec 21 P3) — `backup:all-agents` cron command, configurable destination and retention days via `BackupConfig` schema

### Changed
- CI streamlined: removed duplicate npm audit from nightly (kept in security.yml), fixed dependabot-auto-merge check name (`quality` not `typecheck`), staggered CodeQL to Wednesday, added zod major version ignore
- Dependencies: better-sqlite3 11→12, @types/node 22→25, @vitest/coverage-v8 3→4, actions/setup-python v5→v6

### Fixed
- **Rolldown chunk circular dependency** — dynamic `import("koina/fs.js")` in loader.ts created a chunk that referenced `__exportAll` from entry.mjs, causing `TypeError: __exportAll is not a function` at startup. Converted to static import.
- 96 lint warnings eliminated (unused imports, unsorted imports, unused parameters across 21 test files)
- Removed dead auth scaffolding (tls.ts, sanitize.ts, retention.ts — spec 3 stubs with zero consumers)
- Removed unused `StoreError` class

### Specs Completed
- **Spec 18** Extensibility — 6/6 phases
- **Spec 20** Security Hardening — 4/4 phases

---

## [0.10.0] - 2026-02-20

### Added
- **Session continuity hardening** (Spec 12) — pre-compaction memory flush with distillation log, background session aggressive distillation (50 msg / 10K token triggers), ephemeral session cleanup (nightly purge), post-distillation verification checks
- **Sub-agent workforce** (Spec 13) — 5 typed roles (coder, reviewer, researcher, explorer, runner) with structured JSON result contracts, parallel dispatch via `sessions_dispatch`, per-agent budget controls (turns, tokens, timeout)
- **Domain-scoped memory** — agents filter recall by configured domains, backwards-compatible (unscoped memories always included)
- **Memory confidence scoring** — Neo4j MemoryAccess decay/access counts weight search results, frequently accessed memories boosted, decayed memories penalized
- **Tool categorization UI** — 28 tools mapped to 6 categories (filesystem, search, execute, communication, system, web) with category badges in tool panel header and status line
- **Per-tool token estimates** — token consumption estimated per tool result, displayed in tool panel
- **Distillation progress indicator** — live progress bar in webchat showing pipeline stage (sanitize, extract, summarize, flush, verify)
- **Dynamic thinking budget** — thinking token budget scales with message complexity (2K-16K range)
- **Tool result truncation** — per-tool-type storage limits with head/tail preservation
- **Bootstrap token audit** — `aletheia audit-tokens [agent-id]` CLI command for per-section token breakdown
- **Release automation** — release-please for versioned releases with auto-generated changelogs

### Changed
- Release workflow switched from manual tag-push to release-please managed releases
- Cost injection now runs every 8th turn with session cost, sub-agent costs, budget remaining

### Fixed
- Auth mode "none" not recognized in UI auth flow
- TypeScript errors in audit.ts constructor and sessions-spawn unused parameter

---

## [0.9.1] - 2026-02-20

### Added
- **Working state extraction** - Post-turn LLM extraction of structured task context (current task, completed steps, next steps, decisions, open files). Persisted on session, injected into system prompt, survives distillation.
- **Agent notes tool** - Explicit `note` tool (add/list/delete) with categories (task, decision, preference, correction, context). Notes survive distillation and are injected into system prompt with 2K token cap.
- **Release workflow** - GitHub Releases on tag push with auto-generated changelogs
- **Self-update CLI** - `aletheia update [version]` with locking, rollback, and health-check. Supports `--edge` (latest main), `--check` (dry run), `--rollback`.
- **Version tracking** - Build-time version injection, `/api/status` includes version, 6h periodic update check daemon
- **Thinking UI** - Status pills showing thinking duration during streaming, expandable detail panel with full reasoning
- **2D graph visualization** - Force-graph 2D as default (faster, more readable), lazy-loaded 3D via dynamic import, progressive node loading (batches of 200)
- **Error handling sweep** - `trySafe`/`trySafeAsync` utilities in `koina/safe.ts`, catch block improvements across store, finalize, tools, listener, TTS

### Fixed
- **Duplicate tool_result blocks** - Loop detector was pushing additional `tool_result` blocks with the same `tool_use_id`, causing Anthropic API 400 errors. Now appends to existing result instead.
- **Recall timeouts** - Vector-first search as primary path (~200-500ms), graph-enhanced only as fallback when no vector hits above threshold. Timeout bumped 3s to 5s.
- **Manager test failures** - Restored 27 tests by adding missing store mocks (getThinkingConfig, getNotes, getWorkingState, etc.)

### Changed
- Session metrics (duration, context utilization, distillation count) moved to separate block, injected every 8th turn instead of replacing working state

---

## [0.9.0] - 2026-02-18

### Added
- **Web UI feature pack** - file explorer with syntax-highlighted preview, settings panel, diff rendering in tool results
- **Tool approval system** - risk classification (low/medium/high/critical) with UI approval gates for dangerous operations
- **MCP client support** - stdio, SSE, and HTTP transports for connecting to external MCP servers
- **Prompt caching** - `cache_control` breakpoints on tool definitions and conversation history. Uses 2 of 4 Anthropic cache slots (other 2 on system prompt). 60-80% input token savings during tool loops.
- **Pipeline decomposition** - `manager.ts` broken into composable stages for testability
- **Design specs** - auth/security, data privacy, tool governance, distillation persistence, modular runtime architecture (in `docs/specs/`)

### Fixed
- History endpoint returns newest messages instead of oldest
- WebGPU polyfill for Three.js in non-WebGPU browsers
- Lazy-load GraphView to prevent Three.js crash on startup

---

## [0.8.0] - 2026-02-17

### Added
- **Structured logging** - turn-level tracing with request IDs, tool call timing, cache hit rates
- **Smart loop detection** - warn at 3, halt at 5 identical tool calls (replaces hard cap)
- **Pre-turn memory recall** - Mem0 search injected into context before each agent turn
- **Prosoche signal collectors** - calendar, task, cross-agent signals feed the attention daemon
- **Conversation backfill** - script to import existing session transcripts into the memory pipeline
- **Local embeddings** - Voyage-3-large with Ollama fallback
- **Instance branding** - configurable name, tagline, logo via config
- **Tool governance** - timeouts, turn cancellation, structured approval flow
- **Distillation persistence** - summaries written to agent workspace memory files

### Fixed
- Config env injection via typed `applyEnv()`
- fd binary symlink on Ubuntu CI
- Streaming UI bugs (stuck stop button, collapsed formatting, disappearing messages)

---

## [0.7.0] - 2026-02-16

### Added
- **Graph visualization** - 3D force-directed graph with community detection, progressive loading, Three.js rendering
- **Syntax highlighting** - tree-shaken highlight.js in chat and tool panel
- **Collapsible tool results** - `/compact` command for dense tool output
- **File uploads** - images (vision), PDFs, text files, code files via drag-and-drop
- **3D memory graph** - interactive visualization of entity relationships in the web UI

### Fixed
- Streaming events unbuffered for real-time delivery
- Credential file supports `apiKey` field (not just `authToken`)
- Bootstrap entry point renamed for tsdown v0.20 compatibility

---

## [0.6.0] - 2026-02-15

### Added
- **Web UI** - Svelte 5 chat interface at `/ui`
  - Streaming responses via SSE
  - Real-time event push
  - Per-agent conversation switching
  - Markdown rendering, emoji support
  - Mobile-responsive with swipe sidebar
  - Dark theme
- **Streaming API** - `completeStreaming()` on AnthropicProvider and ProviderRouter
- **Collapsible sidebar** - slash commands (`/new`, `/switch`, `/help`), copy code blocks
- **Live tool activity feed** - real-time tool execution status in UI
- **Event bus to SSE bridge** - turn, tool, session events broadcast to web clients
- **Cost endpoints** - `/api/costs/summary`, `/api/costs/session/:id`

### Changed
- JS bundle: 1,031KB to 199KB (81% reduction via hljs tree-shaking)
- Runtime bundle: 293KB to 354KB
- Static file serving with SPA fallback, immutable caching for hashed assets

---

## [0.5.0] - 2026-02-14

### Added
- **Memory intelligence** - graph analytics (PageRank, Louvain), query rewriting, foresight signals
- **Self-observation tools** - check_calibration, what_do_i_know, recent_corrections
- **Cross-agent deliberation** - structured dialectic protocol between agents
- **Discovery engine** - cross-domain connection finding via shared memory
- **Memory evolution** - merge, reinforce, decay lifecycle for stored facts
- **Temporal memory layer** - Graphiti-style episodic memory in the sidecar
- **Research meta-tool** - memory search, web search, synthesize pipeline
- **Whisper transcription** - wraps whisper.cpp for voice message handling
- **Browser automation** - LLM-driven web browsing via Playwright

### Changed
- Temperature routing based on message content classification
- MCP security hardening (auth, rate limiting, scopes, CORS)
- Cross-agent blackboard (SQLite migration v6, TTL-based expiry)

---

## [0.4.0] - 2026-02-13

### Added
- **CI/CD pipeline** - PR gates, nightly quality checks, deploy pipeline
- **Security scanning** - Dependabot, CodeQL, npm audit, pip-audit, TruffleHog
- **P0 test suite** - git pre-commit hooks, vitest with coverage thresholds
- **TTS pipeline** - Piper voice synthesis for audio responses
- **Circuit breakers** - input/response quality gates on agent turns
- **Ephemeral agents** - spawn short-lived sub-agents for isolated tasks
- **Self-authoring tools** - agents can create new tools at runtime
- **Competence model** - per-agent skill tracking and confidence scoring

### Changed
- Tests: 287 to 689 (80%+ coverage across 68 files)
- Dual licensing: AGPL-3.0 (runtime) + Apache-2.0 (SDK/client)

---

## [0.3.0] - 2026-02-11

### Added
- **Capability rebuild** - Signal commands (10 built-in), link preprocessing, media understanding, CLI admin, contact pairing, skills directory
- **Adaptive routing** - complexity-based model selection with tiered routing
- **Planning tools** - multi-step task decomposition for agents
- **Disagreement detection** - flags conflicting agent responses
- **Session replay CLI** - replay and re-execute past sessions
- **Operational metrics** - `/api/metrics` with per-agent stats, token usage, cache rates, cron status
- **Concurrent agent turns** - fire-and-forget SSE dispatch

### Fixed
- Distillation amnesia: summary marked `isDistilled: true` made it invisible to future turns
- Tool results excluded from extraction/summarization
- Tool definition tokens not subtracted from history budget
- Watchdog pgrep regex never matched gateway process

### Changed
- Token counter: added safety margin (1.15x), per-tool overhead accounting
- Distillation trigger uses actual API-reported input tokens instead of heuristic
- Multi-stage summarization for large conversations
- Heartbeat uses Haiku (95% token savings)

---

## [0.2.0] - 2026-02-08

### Added
- **Runtime v2** - clean-room rewrite, removed all upstream dependencies (789k lines, 47 packages)
- **Stack**: Hono (gateway), better-sqlite3 (sessions), @anthropic-ai/sdk, Zod (config), Commander (CLI)
- **OAuth authentication** - Anthropic Max plan routing with Bearer tokens
- **Mem0 memory integration** - automatic fact extraction via Claude Haiku
  - Qdrant vector store for semantic search
  - Neo4j graph store for entity relationships
  - FastAPI sidecar service (port 8230)
  - Cross-agent shared memory + agent-specific domain memory
- **Federated memory search** - sqlite-vec + Mem0 queried in parallel, merged and deduplicated
- **Prosoche daemon** - adaptive attention engine with configurable signal weights
- **Evaluation framework** - objective metrics and bias monitoring

### Changed
- FalkorDB retired, data migrated to Neo4j
- Memory plugin hooks: before_agent_start (recall), agent_end (extract)
- Config paths: `.openclaw/` to `.aletheia/`

---

## [0.1.0] - 2026-02-05

### Added
- Initial fork from OpenClaw as Aletheia
- Multi-agent architecture with per-agent workspaces
- Signal messaging via signal-cli
- sqlite-vec local memory search
- Structured distillation (pre-compaction fact extraction)
- Context assembly pipeline
- Knowledge graph with ontology
- Research tools (scholar, wiki)
- Langfuse observability integration
