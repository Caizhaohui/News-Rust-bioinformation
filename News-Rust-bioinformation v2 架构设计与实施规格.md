# News-Rust-bioinformation v2  
## Rust Bioinformatics Radar 数据模型、README 与自动化架构设计

> 本文档是供 Gemini / Codex 等代码 Agent 直接执行的工程规格。  
> 目标仓库：`Caizhaohui/News-Rust-bioinformation`

---

# 1. 项目定位

将当前项目从：

> A living catalog of Rust bioinformatics tools

升级为：

> **Rust Bioinformatics Radar — 一个持续发现、分类、评估和追踪 Rust 生物信息学项目的可自动维护知识库。**

项目不应只是 Awesome List。

v2 应能够回答以下问题：

1. Rust 生物信息学目前有哪些主要工具？
2. 哪些项目仍然活跃？
3. 哪些项目正在快速增长？
4. 哪些是 Pure Rust，哪些依赖 C/C++，哪些采用 Python + Rust？
5. 哪些项目适合学习 Rust 生物信息学源码？
6. 最近 30/90 天出现了哪些新项目？
7. 哪些项目最近发布了新版本？
8. Rust 在哪些生物信息学领域发展最快？
9. 每个项目对应哪些论文？
10. 哪些项目已经长期不维护？

---

# 2. 核心设计原则

## 2.1 人工数据与自动数据分离

必须严格区分：

```text
data/tools.yaml
        │
        │ 人工维护
        ▼
项目身份 / 分类 / 描述 / 架构 / 标签 / 论文
        │
        │
        ├──────────────┐
        ▼              ▼
 GitHub API        Crossref / DOI
        │
        ▼
data/metadata.json
        │
        │ 自动生成
        ▼
stars / forks / pushed_at / releases / archived...
```

### `tools.yaml`

属于：

> **Curated Source of Truth**

只保存需要人工判断的信息。

### `metadata.json`

属于：

> **Generated Cache**

只保存能够从 GitHub/API 自动获得的信息。

禁止把以下字段人工维护在 `tools.yaml`：

```yaml
stars:
forks:
watchers:
last_push:
latest_release:
open_issues:
```

这些信息必须由自动程序生成。

---

# 3. 推荐目录结构

目标结构：

```text
News-Rust-bioinformation/
│
├── README.md
├── RADAR.md
├── CONTRIBUTING.md
├── CHANGELOG.md
├── LICENSE
│
├── Cargo.toml
├── Cargo.lock
│
├── data/
│   ├── tools.yaml
│   ├── config.yaml
│   ├── metadata.json
│   ├── trends.json
│   │
│   └── snapshots/
│       ├── 2026-09-01.json
│       ├── 2026-09-08.json
│       └── ...
│
├── schema/
│   ├── tools.schema.json
│   └── config.schema.json
│
├── digest/
│   ├── README.md
│   ├── 2026-09.md
│   └── ...
│
├── discover/
│   ├── candidates.yaml
│   └── rejected.yaml
│
├── docs/
│   ├── ARCHITECTURE.md
│   ├── CATEGORIES.md
│   ├── SCORING.md
│   └── CONTRIBUTING_TO_DATA.md
│
├── src/
│   ├── main.rs
│   ├── model.rs
│   ├── schema.rs
│   ├── github.rs
│   ├── metadata.rs
│   ├── trends.rs
│   ├── scoring.rs
│   ├── render.rs
│   ├── radar.rs
│   ├── digest.rs
│   ├── discover.rs
│   └── validate.rs
│
└── .github/
    ├── workflows/
    │   ├── validate.yml
    │   ├── update-metadata.yml
    │   ├── weekly-radar.yml
    │   └── discover.yml
    │
    └── ISSUE_TEMPLATE/
        └── add-tool.yml
```

不要求一次全部实现。

优先完成 P0/P1 功能。

---

# 4. tools.yaml v2 数据模型

## 4.1 顶层结构

推荐：

```yaml
schema_version: 2

categories:
  # 分类定义

tools:
  # 项目定义
```

不要继续让分类信息隐含在每个项目里而没有统一定义。

---

# 5. Category Schema

示例：

```yaml
schema_version: 2

categories:

  crispr:
    name: CRISPR
    description: CRISPR screening, guide design, genome editing and related tools.
    order: 10

  microbial:
    name: Microbial Bioinformatics
    description: Genomics and bioinformatics tools focused on microorganisms.
    order: 20

    children:

      bacterial-assembly:
        name: Bacterial Genome Assembly
        order: 10

      genome-annotation:
        name: Genome Annotation
        order: 20

      prokaryotic-transcriptome:
        name: Prokaryotic Transcriptome
        order: 30

      metagenomics:
        name: Metagenomics
        order: 40

      phage-defense:
        name: Phage Defense Systems
        order: 50

      antimicrobial-resistance:
        name: Antimicrobial Resistance
        order: 60

      transposons:
        name: Transposon Systems
        order: 70

  core-libraries:
    name: Core Libraries
    order: 30

  sequence-io:
    name: Sequence IO and Formats
    order: 40

  alignment:
    name: Alignment and Mapping
    order: 50

  variants:
    name: Variants and Annotation
    order: 60

  long-reads:
    name: Long Reads
    order: 70

  assembly-pangenome:
    name: Assembly and Pangenomes
    order: 80

  single-cell-rna:
    name: Single-cell and RNA
    order: 90

  proteomics:
    name: Proteomics and Structure
    order: 100

  protein-engineering:
    name: Protein Engineering
    order: 110

  infrastructure:
    name: Workflows and Infrastructure
    order: 120

  visualization:
    name: Visualization
    order: 130

  rust-python:
    name: Rust × Python
    description: Python-facing bioinformatics packages powered partially or primarily by Rust.
    order: 140

  learning:
    name: Learning Resources
    order: 200
```

---

# 6. Tool Schema

每一个工具采用：

```yaml
tools:

  noodles:
    name: noodles

    repository: zaeleus/noodles

    description: >
      Pure Rust bioinformatics I/O libraries covering
      SAM, BAM, CRAM, VCF, FASTA and related formats.

    categories:
      - core-libraries
      - sequence-io

    domains:
      - genomics
      - sequencing
      - file-formats

    architecture:
      rust_role: native
      interfaces:
        - cli
        - library
      foreign_dependencies: []

    technologies:
      - rust
      - async
      - tokio

    formats:
      - FASTA
      - FASTQ
      - SAM
      - BAM
      - CRAM
      - VCF
      - BCF
      - GFF

    learning:
      recommended: true
      level: advanced
      topics:
        - API design
        - zero-copy IO
        - binary formats
        - async IO

    publications: []

    status:
      catalog: active

    notes: null
```

---

# 7. 字段详细定义

## 7.1 必需字段

所有项目至少：

```yaml
name:
repository:
description:
categories:
architecture:
status:
```

---

## 7.2 repository

格式必须为：

```yaml
repository: owner/repository
```

例如：

```yaml
repository: rust-bio/rust-bio
```

不要保存完整：

```text
https://github.com/...
```

README 渲染器自动生成 URL。

优点：

- GitHub API 请求简单
- 数据更标准
- 避免 URL 格式混乱
- repository 可以直接作为唯一标识符

---

# 8. architecture 字段

这是 v2 最重要的新字段之一。

```yaml
architecture:
  rust_role: native
  interfaces:
    - cli
    - library
  foreign_dependencies: []
```

## `rust_role`

只允许：

```text
native
hybrid
binding
experimental
unknown
```

定义：

### native

核心功能主要由 Rust 实现。

例如：

```text
noodles
rust-bio
skani
sylph
```

### hybrid

Rust 与其他语言共同构成核心系统。

典型：

```text
Python + Rust
Rust + C++
```

例如：

```text
sourmash
```

### binding

主要是对非 Rust library 的 binding。

例如：

```text
rust-htslib
    ↓
HTSlib
```

### experimental

实验性质 Rust port/reimplementation。

### unknown

尚未人工确认。

---

# 9. interfaces

允许：

```text
cli
library
python
r
web
workflow
api
wasm
```

例如：

```yaml
architecture:
  rust_role: hybrid

  interfaces:
    - cli
    - python
    - library
```

---

# 10. foreign_dependencies

表示核心是否严重依赖其他语言生态。

例如：

```yaml
architecture:
  rust_role: binding

  foreign_dependencies:
    - HTSlib
```

Pure Rust：

```yaml
foreign_dependencies: []
```

---

# 11. domains

领域标签与分类不同。

分类用于 README hierarchy。

domain 用于搜索、统计和未来网页化。

标准词汇建议：

```text
genomics
microbial-genomics
metagenomics
transcriptomics
single-cell
proteomics
phylogenetics
population-genetics
structural-biology
genome-editing
crispr
sequence-analysis
machine-learning
workflow
visualization
```

---

# 12. formats

可选字段：

```yaml
formats:
  - FASTA
  - FASTQ
  - BAM
  - VCF
```

建议采用固定大小写标准。

允许主要格式：

```text
FASTA
FASTQ
SAM
BAM
CRAM
VCF
BCF
BED
GFF
GTF
PAF
BigWig
BigBed
Newick
PDB
mmCIF
```

不要要求所有项目必须填写。

---

# 13. learning 字段

用于构建：

> Recommended source code

示例：

```yaml
learning:
  recommended: true

  level: intermediate

  topics:
    - k-mer algorithms
    - MinHash
    - rayon
    - CLI architecture
```

`level` 只允许：

```text
beginner
intermediate
advanced
```

---

# 14. publications

建议：

```yaml
publications:

  - doi: 10.1093/bioinformatics/btv573
    title: Rust-Bio: a fast and safe bioinformatics library
    type: journal

  - doi: null
    url: https://www.biorxiv.org/...
    title: Example preprint
    type: preprint
```

允许：

```text
journal
preprint
conference
software-paper
```

论文标题可以人工保存。

未来 DOI metadata 可自动补充。

---

# 15. catalog status

不要与 GitHub activity 混在一起。

人工状态：

```yaml
status:
  catalog: active
```

允许：

```text
active
retired
watch
excluded
```

意义：

### active

正常出现在主 catalog。

### retired

移动至 Retired。

### watch

候选或成熟度不足。

### excluded

明确排除，但为了防止 discovery 重新加入而保留记录。

---

# 16. 自动计算的 activity 状态

不要写入 `tools.yaml`。

根据 `metadata.json` 的 `pushed_at` 自动计算：

```text
🟢 Active
🟡 Maintained
🟠 Quiet
🔴 Inactive
⚫ Archived
```

默认规则：

```text
Active
last push <= 90 days

Maintained
91–365 days

Quiet
366–730 days

Inactive
> 730 days

Archived
GitHub archived == true
```

阈值放入：

```text
data/config.yaml
```

例如：

```yaml
activity:
  active_days: 90
  maintained_days: 365
  quiet_days: 730
```

---

# 17. metadata.json v2

这是程序自动生成文件。

建议结构：

```json
{
  "schema_version": 2,
  "generated_at": "2026-09-10T00:00:00Z",

  "repositories": {

    "zaeleus/noodles": {
      "stars": 720,
      "forks": 65,
      "open_issues": 20,

      "created_at": "...",
      "updated_at": "...",
      "pushed_at": "...",

      "archived": false,
      "disabled": false,

      "default_branch": "master",

      "license": "MIT",

      "topics": [
        "bioinformatics",
        "rust"
      ],

      "language": "Rust",

      "latest_release": {
        "tag": "v0.xx",
        "published_at": "...",
        "url": "..."
      },

      "activity": "active"
    }
  }
}
```

README 只能读取，不允许人工修改。

---

# 18. snapshots 与趋势计算

当前 snapshot 机制应保留并升级。

每周生成：

```text
data/snapshots/YYYY-MM-DD.json
```

只保存趋势计算需要的紧凑字段：

```json
{
  "date": "2026-09-10",

  "repositories": {

    "zaeleus/noodles": {
      "stars": 720,
      "forks": 65
    },

    "rust-bio/rust-bio": {
      "stars": 1845,
      "forks": 315
    }
  }
}
```

不要每次 snapshot 完整复制 metadata。

---

# 19. trends.json

由 snapshot 自动计算：

```json
{
  "generated_at": "...",

  "repositories": {

    "zaeleus/noodles": {

      "stars_7d": 8,
      "stars_30d": 31,
      "stars_90d": 92,

      "growth_30d": 0.045,

      "trend": "rising"
    }
  }
}
```

---

# 20. Emerging project 判定

定义一个自动标签：

```text
🌱 Emerging
```

建议规则：

```text
stars < 150

AND

last push <= 180 days

AND

created_at <= 3 years
```

注意：

Emerging ≠ Quality。

这里只代表：

> relatively new + active + not yet highly starred

---

# 21. Trending 判定

定义：

```text
🔥 Trending
```

第一版不要设计复杂 AI score。

采用透明规则：

```text
stars gained in 30 days >= 10
```

或者：

```text
30-day growth >= 10%
AND
stars >= 20
```

以后再优化。

---

# 22. Learning 推荐

不要自动计算。

只来自：

```yaml
learning:
  recommended: true
```

README 显示：

```text
📖 Source Pick
```

原因是：

> star 数无法代表源码教学价值。

---

# 23. README v2 页面结构

README 不再只是超长分类列表。

推荐：

```markdown
# Rust Bioinformatics Radar 🦀🧬

Introduction

Badges

## At a Glance

## 🔥 Trending

## 🌱 Emerging Projects

## 📖 Source Code Picks

## 🧬 Bioinformatics Applications

### CRISPR

### Microbial Bioinformatics

...

## 🦀 Core Rust Libraries

## 🐍 Rust × Python

## ⚙️ Infrastructure

## 📊 Project Activity

## 📰 Latest Radar

## 📚 Learning Resources

## 🤝 Contributing

## Methodology
```

---

# 24. README Header

建议：

```markdown
# Rust Bioinformatics Radar 🦀🧬

A continuously updated catalog and ecosystem radar for
Rust libraries, applications and infrastructure in
bioinformatics and computational biology.

> Automatically tracks repository activity, releases,
> ecosystem trends and emerging projects.
```

然后：

```text
Projects: xxx
Active: xxx
Pure Rust: xxx
Rust × Python: xxx
Updated: YYYY-MM-DD
```

这些数字全部自动生成。

---

# 25. At a Glance

自动生成：

```markdown
## Ecosystem at a Glance

| Metric | Count |
|---|---:|
| Projects | 235 |
| Active projects | 171 |
| Pure Rust | 142 |
| Rust × Python | 26 |
| Published software | 58 |
| Emerging projects | 31 |
```

不要人工维护数字。

---

# 26. Trending

只展示 5–10 个：

```markdown
## 🔥 Trending

Projects showing notable GitHub growth during the last 30 days.

| Project | Area | Stars | +30d | Activity |
|---|---|---:|---:|---|
| xxx | Metagenomics | 100 | +27 | 🟢 |
```

目的是让用户打开 README 后第一屏就看到：

> Rust bioinformatics 最近发生了什么。

---

# 27. Emerging

```markdown
## 🌱 Emerging Projects

Recently active Rust bioinformatics projects that are still
relatively small but worth watching.
```

最多 10–15 个。

完整 catalog 后面再展示。

---

# 28. Source Code Picks

例如：

```markdown
## 📖 Recommended Source Code

| Project | Learn |
|---|---|
| noodles | Bioinformatics IO architecture |
| rust-bio | Algorithms and Rust API design |
| skani | High-performance microbial genomics |
| sylph | MinHash and metagenomics |
| sourmash | Python + Rust architecture |
| rasusa | Clean Rust CLI design |
```

全部来自 `learning` 字段。

---

# 29. Catalog 行格式

不要让每个项目占 5–10 行。

继续使用紧凑格式。

推荐：

```markdown
- **[noodles](...)** — Pure Rust bioinformatics I/O for BAM,
  CRAM, VCF and FASTX.
  `🦀 Native` `🟢 Active` `★ 720` `📖 Source Pick`
```

或者表格。

但建议：

### 一级领域用表格

### 大型完整 catalog 保持 bullet list

原因：

GitHub Markdown 巨型 table 可读性较差。

---

# 30. Architecture badges

README 自动生成：

```text
🦀 Native
🐍 Rust × Python
🔗 Binding
🧪 Experimental
```

映射：

```text
native       → 🦀 Native
hybrid       → 🐍 Hybrid
binding      → 🔗 Binding
experimental → 🧪 Experimental
```

如果 hybrid 不一定 Python：

程序检查：

```text
interfaces contains python
```

则：

```text
🐍 Rust × Python
```

否则：

```text
🔀 Hybrid
```

---

# 31. Activity badge

```text
🟢 Active
🟡 Maintained
🟠 Quiet
🔴 Inactive
⚫ Archived
```

---

# 32. README 不应直接展示所有 metadata

避免：

```text
stars
forks
issues
watchers
release
updated
created
```

全塞进去。

README 是导航。

详细 metadata 留给 JSON / 后续 website。

---

# 33. RADAR.md 定位

README：

> 当前生态地图

RADAR.md：

> 最近发生了什么

自动生成：

```markdown
# Rust Bioinformatics Radar

Generated: 2026-09-10

## New Projects

## Trending

## New Releases

## Recently Revived

## Became Inactive

## Archived

## New Publications
```

---

# 34. Monthly Digest

保留：

```text
digest/YYYY-MM.md
```

例如：

```markdown
# Rust Bioinformatics Digest — September 2026

## Highlights

## New Projects

## Major Releases

## Rising Projects

## Publications

## Microbial Genomics

## Long-read Sequencing

## Rust × Python
```

第一阶段可以完全 rule-based 自动生成。

不要要求 LLM 才能运行核心 pipeline。

未来 AI summary 只能作为 optional enhancement。

---

# 35. CLI 架构

当前 Rust CLI 继续作为核心引擎。

建议统一：

```bash
nrb validate

nrb fetch-metadata

nrb snapshot

nrb trends

nrb build-readme

nrb build-radar

nrb build-digest

nrb discover

nrb check
```

---

# 36. 推荐命令行为

## validate

```bash
nrb validate
```

检查：

- YAML syntax
- schema_version
- duplicate repository
- duplicate project id
- invalid category
- invalid architecture enum
- invalid DOI
- malformed repository
- unknown domain
- unknown format
- required fields
- retired project consistency

错误必须：

```text
exit code != 0
```

方便 CI 阻止 merge。

---

# 37. check

```bash
nrb check
```

运行：

```text
validate
+
build README into memory/temp
+
compare generated README
+
compare RADAR
```

如果 generated file 与 repository 不一致：

```text
exit 1
```

用于 PR CI。

---

# 38. fetch-metadata

```bash
nrb fetch-metadata
```

行为：

```text
tools.yaml
    ↓
collect unique GitHub repositories
    ↓
GitHub REST API
    ↓
metadata.json
```

必须：

- 支持 GitHub token
- Respect rate limit
- 遇到单仓库失败不能破坏整个 metadata
- 保留上一版本有效 metadata
- 输出 warning
- 使用 deterministic JSON formatting

---

# 39. snapshot

```bash
nrb snapshot
```

如果当天 snapshot 已存在：

默认 overwrite 或跳过需要确定一种。

推荐：

```text
same date → overwrite
```

GitHub Actions 每周只运行一次，因此不会造成问题。

---

# 40. trends

```bash
nrb trends
```

读取：

```text
metadata.json
+
snapshots
```

产生：

```text
data/trends.json
```

支持：

```text
7d
30d
90d
```

找不到精确日期 snapshot 时：

使用最接近日期。

---

# 41. GitHub Actions 设计

至少拆成三个 workflow。

不要做一个巨大 workflow。

---

# 42. Workflow 1 — validate.yml

触发：

```yaml
on:
  pull_request:
  push:
    branches:
      - main
```

目标：

> 所有代码和 catalog 修改都必须通过质量检查。

逻辑：

```yaml
jobs:

  validate:

    runs-on: ubuntu-latest

    steps:

      - checkout

      - install Rust stable

      - cargo fmt --check

      - cargo clippy -- -D warnings

      - cargo test

      - cargo run -- validate

      - cargo run -- build-readme --check
```

如果 CLI 最终提供：

```bash
nrb check
```

则：

```yaml
cargo run -- check
```

即可。

---

# 43. Workflow 2 — update-metadata.yml

触发：

```yaml
on:

  schedule:
    - cron: "17 3 * * *"

  workflow_dispatch:
```

即每天运行一次。

注意 cron 使用 UTC。

步骤：

```text
checkout
↓
build nrb
↓
fetch metadata
↓
build README
↓
build RADAR
↓
git diff
↓
commit only if changed
```

GitHub 权限：

```yaml
permissions:
  contents: write
```

环境变量：

```yaml
GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

提交 message：

```text
chore(data): refresh repository metadata
```

---

# 44. 避免无意义 commit

必须：

```bash
git diff --quiet
```

无变化：

```text
exit success
```

禁止每天制造空 commit。

此外 README 中若包含精确 timestamp，会导致每天必然变化。

因此 README 最多显示：

```text
Last updated: 2026-09-10
```

而不是秒级 timestamp。

---

# 45. Workflow 3 — weekly-radar.yml

触发：

```yaml
schedule:
  weekly
```

建议：

```text
每周一 UTC 04:00
```

流程：

```text
fetch metadata
      ↓
snapshot
      ↓
trends
      ↓
build radar
      ↓
build README
      ↓
commit
```

commit：

```text
chore(radar): update weekly Rust bioinformatics radar
```

---

# 46. Workflow 4 — discover.yml

这是 P2 功能。

不要影响第一版上线。

建议每周运行：

```text
GitHub search
     ↓
candidate repositories
     ↓
filter existing tools
     ↓
discover/candidates.yaml
```

关键词：

```text
language:Rust bioinformatics

language:Rust genomics

language:Rust sequencing

language:Rust metagenomics

language:Rust FASTQ

language:Rust BAM

language:Rust CRISPR

language:Rust proteomics
```

但不要自动加入 `tools.yaml`。

这是一个关键原则：

> Discovery 可以自动化，Curating 不自动化。

候选项目只能进入：

```text
discover/candidates.yaml
```

由维护者审核。

---

# 47. Candidate Schema

```yaml
candidates:

  owner/repo:

    discovered_at: 2026-09-10

    source:
      type: github-search
      query: bioinformatics language:Rust

    github:
      stars: 12
      description: "..."

    decision: pending
```

decision：

```text
pending
accepted
rejected
```

如果 rejected：

```yaml
decision: rejected

reason:
  - not-bioinformatics
```

避免未来反复发现。

---

# 48. config.yaml v2

建议：

```yaml
schema_version: 2

activity:
  active_days: 90
  maintained_days: 365
  quiet_days: 730

emerging:
  max_stars: 150
  max_age_days: 1095
  max_inactive_days: 180

trending:
  min_stars: 20
  min_gain_30d: 10
  min_growth_30d: 0.10

readme:
  trending_limit: 10
  emerging_limit: 12
  source_picks_limit: 12

snapshot:
  keep_days: 730
```

所有阈值禁止硬编码在 Rust source 中。

---

# 49. JSON Schema

新增：

```text
schema/tools.schema.json
```

即便 Rust 本身已经通过 Serde struct 校验，也建议提供 JSON Schema。

目的：

- IDE 自动提示
- GitHub PR 校验
- AI Agent 更容易理解数据格式
- 外部 contributor 更容易编辑
- schema version 可管理

---

# 50. Rust Model

建议使用：

```rust
#[derive(Debug, Deserialize, Serialize)]
struct Catalog {
    schema_version: u32,
    categories: IndexMap<String, Category>,
    tools: IndexMap<String, Tool>,
}
```

Tool：

```rust
struct Tool {
    name: String,
    repository: String,
    description: String,

    categories: Vec<String>,

    #[serde(default)]
    domains: Vec<String>,

    architecture: Architecture,

    #[serde(default)]
    technologies: Vec<String>,

    #[serde(default)]
    formats: Vec<String>,

    #[serde(default)]
    publications: Vec<Publication>,

    #[serde(default)]
    learning: Option<Learning>,

    status: Status,

    #[serde(default)]
    notes: Option<String>,
}
```

不要为了 v2 引入数据库。

YAML + JSON 足够。

---

# 51. Sorting 规则

当前项目按 star 排序的设计可以保留，但稍作明确。

每个 category：

```text
1. catalog status active
2. stars DESC
3. pushed_at DESC
4. name ASC
```

Trending / Emerging 单独采用各自 score。

---

# 52. 同一工具多分类

允许：

```yaml
categories:
  - microbial/metagenomics
  - long-reads
```

但 README 默认需要一个 primary category，否则会大量重复。

推荐模型进一步调整为：

```yaml
categories:
  primary: microbial/metagenomics

  secondary:
    - long-reads
```

因此最终建议采用：

```yaml
category:
  primary: microbial/metagenomics
  secondary:
    - long-reads
```

而不是简单 `categories:`。

README Catalog 只在 primary 分类出现一次。

Tags/Search 可以使用 secondary。

这是推荐的最终方案。

---

# 53. tools.yaml 最终推荐示例

```yaml
schema_version: 2

categories:

  microbial:
    name: Microbial Bioinformatics
    order: 20

    children:

      metagenomics:
        name: Metagenomics
        order: 40

  core-libraries:
    name: Core Libraries
    order: 30

  sequence-io:
    name: Sequence IO and Formats
    order: 40

tools:

  noodles:

    name: noodles

    repository: zaeleus/noodles

    description: >
      Pure Rust bioinformatics I/O libraries covering SAM,
      BAM, CRAM, VCF, FASTA and related formats.

    category:
      primary: core-libraries
      secondary:
        - sequence-io

    domains:
      - genomics
      - sequence-analysis

    architecture:
      rust_role: native

      interfaces:
        - library

      foreign_dependencies: []

    technologies:
      - rust
      - tokio

    formats:
      - FASTA
      - FASTQ
      - SAM
      - BAM
      - CRAM
      - VCF
      - BCF

    learning:
      recommended: true
      level: advanced

      topics:
        - bioinformatics IO
        - API design
        - binary file formats
        - async Rust

    publications: []

    status:
      catalog: active
```

---

# 54. Rust × Python 专题

README 必须新增：

```markdown
## 🐍 Rust × Python
```

筛选逻辑：

```text
architecture.interfaces contains python
```

或者：

```text
architecture.rust_role == hybrid
AND
interfaces contains python
```

显示：

```text
sourmash
dna_parser
...
```

这一分类是项目区别于传统 Awesome List 的重要特色。

---

# 55. Microbial Bioinformatics 保持重点

项目应继续强化 microbial 部分。

至少保留：

```text
Microbial Bioinformatics
│
├── Bacterial Genome Assembly
├── Genome Annotation
├── Prokaryotic Transcriptome
├── Metagenomics
├── Comparative Genomics
├── Phylogenetics
├── Phage
├── Phage Defense Systems
├── Antimicrobial Resistance
└── Mobile Genetic Elements
    ├── Transposons
    └── Plasmids
```

建议将当前：

```text
Transposon Systems
```

未来扩展成：

```text
Mobile Genetic Elements
```

这样可以覆盖：

- IS elements
- transposons
- integrons
- plasmids
- ICE
- prophage

---

# 56. Genome Editing 分类扩展

CRISPR 不建议长期只有一个扁平列表。

结构建议：

```text
Genome Editing / CRISPR
│
├── Guide Design
├── CRISPR Screens
├── Off-target Analysis
├── CRISPR Array Detection
├── Genome Editing Analysis
└── Genome Editing Technologies
```

允许目前项目较少时先不显示空 subsection。

Renderer 必须：

> empty category 不输出。

---

# 57. README 渲染必须 deterministic

同一份：

```text
tools.yaml
metadata.json
trends.json
```

运行 100 次：

```text
README.md SHA 应完全一致
```

不能：

- 随机排序
- 输出秒级当前时间
- 不稳定 HashMap iteration
- 依赖 API 返回顺序

Rust 建议使用：

```text
BTreeMap
```

或者显式排序。

---

# 58. CONTRIBUTING v2

贡献者新增项目时，应只需要修改：

```text
data/tools.yaml
```

PR checklist：

```markdown
- [ ] Repository is substantially written in Rust or has a meaningful Rust core
- [ ] Project is relevant to bioinformatics/computational biology
- [ ] Description is factual and concise
- [ ] Primary category is appropriate
- [ ] Architecture type is specified
- [ ] Project is not already listed
- [ ] Publication DOI added when available
```

metadata 不要求 contributor 填。

---

# 59. GitHub Issue Form

新增：

```text
.github/ISSUE_TEMPLATE/add-tool.yml
```

字段：

```text
Project name

Repository

Description

Category

Why should it be included?

Rust architecture:
[ ] Native Rust
[ ] Rust + Python
[ ] Rust binding
[ ] Other

Publication DOI

Additional notes
```

---

# 60. 数据迁移策略

不要一次重写所有项目 metadata。

实施脚本：

```bash
nrb migrate-v2
```

或者一次性 migration script。

转换：

```text
旧 category
        ↓
new primary category

旧 papers
        ↓
publications

现有 description
        ↓
description

现有 repository/url
        ↓
repository owner/name
```

新字段无法可靠推断：

```text
architecture
domains
learning
```

默认：

```yaml
architecture:
  rust_role: unknown
  interfaces: []
  foreign_dependencies: []
```

后续人工补充。

不要让 AI 根据项目名字大规模猜 architecture。

---

# 61. 兼容性优先级

迁移期间：

### 必须保留

```text
现有项目
现有论文
现有分类含义
现有 metadata fetching
现有 README 自动生成能力
```

### 可以改变

```text
YAML schema
README layout
CLI module organization
metadata JSON schema
workflow structure
```

---

# 62. 实施阶段

## Phase 1 — Schema Foundation

P0：

```text
tools.yaml schema v2
Rust structs
validation
migration
README compatibility
```

完成标准：

```bash
cargo test
nrb validate
nrb build-readme
```

全部成功。

---

## Phase 2 — Architecture & Activity

P0：

实现：

```text
rust_role
interfaces
activity calculation
badges
Rust × Python
Source Pick
```

---

## Phase 3 — Metadata Automation

P1：

```text
GitHub metadata
latest releases
activity
daily action
```

---

## Phase 4 — Trends

P1：

```text
snapshot
7/30/90d growth
trending
emerging
RADAR.md
```

---

## Phase 5 — Discovery

P2：

```text
GitHub search
candidate system
review workflow
```

---

## Phase 6 — Optional Web Frontend

P3。

暂时不要实现。

未来可以把：

```text
tools.yaml
metadata.json
trends.json
```

直接作为静态网站数据源。

因此 v2 schema 应保持 frontend-friendly。

---

# 63. 不要实现的功能

第一阶段明确禁止 Agent 擅自加入：

```text
PostgreSQL
SQLite
React
Next.js
server backend
Docker dependency
LLM mandatory pipeline
vector database
complex recommendation engine
user accounts
```

当前目标：

> GitHub-native + Rust-native + static data architecture

必须保持项目轻量。

---

# 64. 自动化总体架构

最终数据流：

```text
                    tools.yaml
                       │
                       │ curated
                       ▼
                    validate
                       │
             ┌─────────┴───────────┐
             │                     │
             ▼                     ▼
        GitHub API             Publication
             │                   metadata
             │                     │
             └──────────┬──────────┘
                        ▼
                 metadata.json
                        │
               ┌────────┴────────┐
               │                 │
               ▼                 ▼
            snapshot          activity
               │
               ▼
          historical data
               │
               ▼
            trends
               │
               ▼
           trends.json
               │
      ┌────────┼─────────┐
      ▼        ▼         ▼
 README.md  RADAR.md   digest/
```

---

# 65. 数据职责边界

这是 Agent 必须遵守的核心规则。

```text
tools.yaml
= WHAT the project is

metadata.json
= WHAT GitHub says now

snapshots/
= WHAT GitHub said before

trends.json
= HOW the project changed

README.md
= HUMAN-FRIENDLY CURRENT VIEW

RADAR.md
= WHAT CHANGED RECENTLY

digest/
= PERIODIC HISTORY
```

禁止职责混淆。

---

# 66. 测试

至少添加：

## Unit tests

```text
activity classification

repository parsing

category validation

architecture enum

trend calculation

sorting

markdown escaping
```

## Golden tests

推荐：

```text
tests/fixtures/catalog.yaml
tests/fixtures/metadata.json
tests/expected/README.md
```

测试：

```text
renderer(input) == expected README
```

这样 Agent 后续重构时不容易破坏页面。

---

# 67. Error Handling

CLI 建议：

```text
anyhow
thiserror
```

用户错误必须显示：

```text
data/tools.yaml:132

unknown primary category:
"microbiology"

Did you mean:
"microbial"?
```

而不是：

```text
thread panicked
```

---

# 68. Logging

默认：

```text
INFO
```

支持：

```bash
RUST_LOG=debug nrb fetch-metadata
```

可以采用：

```text
tracing
tracing-subscriber
```

但不是强制。

---

# 69. README 生成注释

README 顶部保留类似：

```markdown
<!--
Generated by nrb.

Do not edit the catalog manually.

Edit:
  data/tools.yaml

Then run:
  nrb build-readme
-->
```

避免 contributor 直接修改生成文件。

---

# 70. Git 提交策略

自动任务统一：

```text
chore(data): refresh repository metadata

chore(radar): update weekly ecosystem radar

chore(digest): generate monthly digest
```

如果 metadata 没有实际变化：

禁止 commit。

---

# 71. Security

GitHub Actions：

只使用：

```text
GITHUB_TOKEN
```

不把 token 写入：

```text
logs
metadata.json
snapshots
```

workflow 权限采用最小权限。

Validation：

```yaml
permissions:
  contents: read
```

更新：

```yaml
permissions:
  contents: write
```

---

# 72. README 最终视觉逻辑

用户打开仓库应该依次看到：

```text
Rust Bioinformatics Radar
          │
          ▼
       Overview
          │
          ▼
   🔥 What's hot?
          │
          ▼
   🌱 What's new?
          │
          ▼
 📖 What should I read?
          │
          ▼
 🧬 What tools exist?
          │
          ▼
 🦀 What libraries exist?
          │
          ▼
 🐍 Rust × Python
          │
          ▼
       Radar/Digest
```

而不是打开 README 后马上看到数百个软件。

---

# 73. 项目的核心差异化

v2 的竞争优势不是：

> We have more links.

而应该是：

```text
Awesome List
     +
GitHub activity tracker
     +
Rust architecture map
     +
Bioinformatics taxonomy
     +
Emerging project radar
     +
Learning guide
```

最终定位：

> **A curated and continuously updated map of the Rust bioinformatics ecosystem.**

---

# 74. Gemini 执行要求

请按照以下顺序执行。

### Step 1

完整阅读当前仓库：

```text
README.md
PLAN.md
RADAR.md
CONTRIBUTING.md
Cargo.toml
src/
data/
.github/workflows/
scripts/
discover/
digest/
```

不要根据本文档假设当前实现。

### Step 2

输出：

```text
V2_IMPLEMENTATION_PLAN.md
```

列出：

```text
current architecture
schema differences
files to modify
migration risks
implementation order
```

### Step 3

实现 v2 schema 和 migration。

不要先修改 README UI。

### Step 4

实现 validation 和 tests。

### Step 5

迁移现有 `tools.yaml`。

确认：

```text
existing tool count is unchanged
existing publications are preserved
```

### Step 6

升级 metadata。

### Step 7

实现 activity / architecture badges。

### Step 8

实现 README v2。

### Step 9

实现 snapshot + trends + RADAR。

### Step 10

升级 GitHub Actions。

### Step 11

运行：

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo run -- validate
cargo run -- build-readme
```

### Step 12

检查：

```bash
git diff
```

并输出最终：

```text
IMPLEMENTATION_SUMMARY.md
```

说明：

```text
implemented features
schema migration
new commands
workflow changes
tests
known limitations
future work
```

---

# 75. Gemini 关键约束

Agent 不得：

1. 删除已有项目。
2. 删除已有 publication 信息。
3. 凭项目名称猜测科学用途。
4. 自动把 discovery candidate 加入正式 catalog。
5. 将 GitHub stars 等动态数据写回 tools.yaml。
6. 为了方便而改用 Python 重写当前 Rust CLI。
7. 引入数据库。
8. 引入前端框架。
9. 引入必须依赖 LLM 的自动化流程。
10. 因迁移 schema 而破坏已有 README 内容覆盖范围。

遇到不确定的 architecture：

```yaml
rust_role: unknown
```

不要猜。

---

# 76. Acceptance Criteria

v2 完成必须满足以下条件。

```text
[ ] tools.yaml schema_version == 2

[ ] schema/tools.schema.json exists

[ ] all projects validate

[ ] no existing project lost

[ ] no existing publication lost

[ ] GitHub metadata is separate from curated YAML

[ ] activity status is automatically calculated

[ ] architecture badges are supported

[ ] Rust × Python view exists

[ ] Source Picks supported

[ ] README overview automatically generated

[ ] README Trending section automatically generated

[ ] README Emerging section automatically generated

[ ] snapshots are generated

[ ] 30-day trends can be calculated

[ ] RADAR.md can be generated

[ ] PR validation workflow exists

[ ] metadata refresh workflow exists

[ ] weekly radar workflow exists

[ ] generated output is deterministic

[ ] cargo fmt passes

[ ] cargo clippy passes

[ ] cargo test passes

[ ] nrb validate passes
```

---

# 77. 最低可用版本定义

如果工作量较大，不要为了完成所有功能而牺牲稳定性。

v2.0 MVP 只要求：

```text
tools.yaml v2
      +
schema validation
      +
architecture type
      +
activity status
      +
README v2
      +
GitHub metadata updater
      +
CI validation
```

以下可以进入 v2.1：

```text
trends
snapshots upgrade
RADAR enhancements
Emerging
Trending
```

以下进入 v2.2：

```text
automatic discovery
monthly digest enhancements
publication metadata enrichment
```

---

# 78. 推荐版本路线

```text
v2.0
│
├── Data model
├── Validation
├── Architecture
├── Activity
└── README redesign
        │
        ▼
v2.1
│
├── Snapshots
├── Trends
├── Trending
├── Emerging
└── Weekly Radar
        │
        ▼
v2.2
│
├── Discovery
├── Publications
├── Monthly digest
└── Ecosystem analytics
        │
        ▼
v3
│
└── Optional static website
```

---

# 79. 最终目标

完成 v2 后，News-Rust-bioinformation 应从：

```text
Rust bioinformatics links
```

升级为：

```text
               Rust Bioinformatics Radar
                         │
          ┌──────────────┼──────────────┐
          │              │              │
       Catalog         Radar          Learning
          │              │              │
      projects        trends        source picks
          │              │              │
      taxonomy        releases      architecture
          │              │              │
      papers          emerging      Rust × Python
```

项目的核心数据资产是：

```text
data/tools.yaml
```

核心程序资产是：

```text
nrb
```

核心用户入口是：

```text
README.md
```

核心持续追踪资产是：

```text
metadata.json
snapshots/
trends.json
RADAR.md
```

整个系统必须保持：

> **Curated, reproducible, deterministic, GitHub-native, Rust-native and easy to contribute to.**