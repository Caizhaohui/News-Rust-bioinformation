# 微生物基因编辑与基因组：值得用 Rust 做的方向

依据本目录缺口、两篇 Science 防御论文的计算瓶颈，以及微生物编辑真实工作流（设计 → 递送 → 验证）。原则：只推荐 CPU 热路径或现有工具在规模上撑不住的环节；不推荐重写 HMMER / ESM / DefenseFinder。

| | |
| --- | --- |
| **优先做** | 编辑设计 + 编辑验证 + 泛基因组脱靶 |
| **可加速** | 注释内循环、IS、质粒/前噬菌体边界 |
| **不要做** | 防御 LM、完整 Bakta/HMM 管线 |

> **和哺乳动物 CRISPR 不是同一套缺口。** 目录里的 guide-counter / sgcount / crispr_screen 服务的是人源筛选 FASTQ。微生物编辑的瓶颈是：在菌种泛基因组上设计 gRNA/CRISPRi、避开 RM 系统、以及用亲本 WGS 验证编辑（breseq 一类）。单基因组上扫 PAM 在 Python 里也不慢。

## A. 微生物基因编辑：建议开发的工具

| 优先级 | 做什么 | 对标谁 | 为什么是 Rust | 不要做成什么 |
| --- | --- | --- | --- | --- |
| P0 | 泛基因组感知的原核 gRNA / CRISPRi 设计器：PAM 扫描、spacer 唯一性、操纵子/TSS、必需基因回避 | GuideMaker、CRISPy-web、CHOPCHOP（均为 Python/Web，且偏单基因组） | 对一株菌不大；对 10³ 株大肠/MAG/噬菌体鸡尾酒是字符串+索引热路径 | 不要做成 Doench 哺乳动物打分器的复刻 |
| P0 | 编辑验证：亲本 vs 突变株短/长读长 WGS，报告预期位点、新连接、混群频率 | breseq（C++/Perl，微生物编辑金标准）、snippy | 每株全基因组比对+新 junction 搜索，实验室常批量几十株 | 不要做成 CRISPResso 短扩增子的完整克隆（价值较小） |
| P0 | 细菌/噬菌体泛基因组脱靶枚举（含 bulge 可选） | Cas-OFFinder（C++/OpenCL）；Rust 移植仍标明不可用 | 一次查询扫上千个细菌基因组，适合 SIMD/索引 | 不要停在单染色体玩具实现 |
| P1 | 多重编辑 / MAGE / 寡核苷酸文库设计（错配、二级结构、基因组唯一性） | MODEST、Merlin 等 Python/Perl | 大量短序列枚举与过滤，CLI 静态分发有优势 | 不要做湿实验协议管理系统 |
| P1 | 构建设递送兼容性：RM 位点、限制酶、已知防御对转化 DNA 的切割风险 | REBASE 查询、经验性 Python 脚本；DefenseFinder 本身不必重写 | 在注释 GFF 上做位点扫描即可；可调用已有 HMM 结果当输入 | 不要重写 DefensePredictor / ESM_DF |

## B. 微生物基因组：编辑的上游，建议补的引擎

编辑设计依赖「基因组上基因、操纵子、移动元件、质粒/前噬菌体在哪」。目录里组装（Autocycler / Polypolish）和 sketch（skani / ska / sourmash）已强；缺的是注释与移动元件。

| 优先级 | 做什么 | 目录现状 | 对标 | Rust 切入点 |
| --- | --- | --- | --- | --- |
| P0 | 生产级插入序列 / 转座子扫描 | rust-ise ★0，ISEScan 风格但未起量 | ISEScan（Python+HMM） | 把 rust-ise 做到与 ISEScan 结果可对拍；编辑要避开 IS 热点 |
| P1 | 质粒与前噬菌体区间划分（覆盖度 + 连接 + sketch，而不是先上 HMM） | 无；噬菌体防御分类为空 | geNomad（神经网络）、MOB-suite、VirSorter2 | 用目录已有的 sourmash/skani 思路做边界与分类；ML 模型保持 Python |
| P1 | 原核基因预测做成 Bakta 可插拔后端 | orphos、dynamite、FragGeneScanRs 都小 | Prodigal、Bakta 的基因预测步 | 加速基因调用；功能注释仍接 HMMER/Diamond，不要重写整库 |
| P1 | 编辑靶点在种内保守性：基因有无、核心/附属、等位基因 | ska.rust、skani、galah 已有距离/去冗余 | Panaroo、PPanGGOLiN、Roary | 在已有 split-kmer 上做「这个 spacer 在 200 株里是否唯一且保守」 |
| P2 | 细菌操纵子 / TU 从 RNA-seq 或邻接规则推断 | 仅 trackclusterTU ★4 | Operon-mapper、Rockhopper | CRISPRi 极性效应需要 TU；数据量中等，优先做对的模型再谈语言 |

## C. 建议的最小产品组合（可做成 2–3 个 CLI）

### microguide（先做）

- **输入：** 基因组 FASTA + GFF（或先跑 orphos）。
- **输出：** Cas9/Cas12/Cas13 / dCas CRISPRi 候选、种内脱靶、操纵子位置。
- **第二期：** 对 GTDB 代表基因组或用户提供的 100–1000 株 pangenome 做脱靶。

### edit-diff（先做）

- **输入：** 亲本与编辑株的 FASTQ/BAM。
- **输出：** 目标位点等位基因频率、非预期 SNV/indel、新接合（质粒丢失、大片段缺失）。
- 对标 breseq 的报告结构，长读长走现有 BAM 工具。

### mobile-scan（随后）

- IS/转座子 + 质粒/前噬菌体区间，供 gRNA 回避和遗传稳定性评估。
- 在 rust-ise 上补齐；前噬菌体用覆盖度和比对，不重写 geNomad。

## D. 明确不要用 Rust 开的方向

| 方向 | 原因 |
| --- | --- |
| DefenseFinder / DefensePredictor / ESM_DF | 瓶颈在 HMMER（已很快）或 GPU 上的 ESM，重写 Python 胶水没有编辑场景收益 |
| 完整 Bakta / PGAP / CRISPRCasTyper | 价值在 HMM 库与分型模型；diced 只覆盖 array。最多做基因预测或 I/O 加速 |
| 蛋白语言模型、结构预测、Cas 蛋白从头设计 | PyTorch 生态；与「菌株编辑工具」不是同一产品 |
| 再做一个 MAGeCK count | 目录已有 guide-counter / sgcount，且面向哺乳动物筛选而非微生物编辑 |

目录已可复用：Autocycler / Polypolish（组装）、orphos / dynamite（基因预测）、diced（array）、plascad（质粒引物）、ska / skani / sourmash（种内距离）。编辑工具应接这些，而不是从零再做组装器。
