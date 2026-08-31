# 新工具候选 · 2026-08-30

手动发现报告，不是收录清单。勾选后再改 `data/tools.yaml`。不要手改 README / RADAR。

## 收录门槛（2026-08-31）

范围：GitHub 上、以 Rust 为主的计算生物学工具。

满足下面 **至少一条** 即保留：

1. **有文献**：预印本（bioRxiv / medRxiv / arXiv 等）或正式期刊论文。
2. **近 1 年仍在维护**：最后一次 push 不早于 2025-08-31。
3. **★ ≥ 50**。

本批 GitHub 检索条件是 `pushed:>=2026-07-31`，因此几乎全部命中都满足第 2 条。文献来自 README / CITATION.cff / Europe PMC，并去掉只引用该工具、并非介绍该工具本身的文章。

通过门槛 **171** 个：有文献 **40** 个；无文献、靠维护或星标 **131** 个；其中 ★≥50 的 **20** 个。

## 有文献

### 1. salmon

- url: [https://github.com/COMBINE-lab/salmon](https://github.com/COMBINE-lab/salmon)
- repo: `COMBINE-lab/salmon`
- 说明: 🐟 🍣 🍱 Highly-accurate & wicked fast transcript-level quantification from RNA-seq reads using selective alignment
- GitHub: ★ 927；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Salmon provides fast and bias-aware quantification of transcript expression.](https://doi.org/10.1038/nmeth.4197)（Nat Methods, 2017）


### 2. cellranger

- url: [https://github.com/10XGenomics/cellranger](https://github.com/10XGenomics/cellranger)
- repo: `10XGenomics/cellranger`
- 说明: 10x Genomics Single Cell Analysis
- GitHub: ★ 474；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Massively parallel digital transcriptional profiling of single cells.](https://doi.org/10.1038/ncomms14049)（Nat Commun, 2017）


### 3. Bismark

- url: [https://github.com/FelixKrueger/Bismark](https://github.com/FelixKrueger/Bismark)
- repo: `FelixKrueger/Bismark`
- 说明: A tool to map bisulfite converted sequence reads and determine cytosine methylation states
- GitHub: ★ 460；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Bismark: a flexible aligner and methylation caller for Bisulfite-Seq applications.](https://doi.org/10.1093/bioinformatics/btr167)（Bioinformatics, 2011）


### 4. cramino

- url: [https://github.com/wdecoster/cramino](https://github.com/wdecoster/cramino)
- repo: `wdecoster/cramino`
- 说明: A *fast* tool for BAM/CRAM quality evaluation, intended for long reads
- GitHub: ★ 189；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [NanoPack2: Population scale evaluation of long-read sequencing data](https://www.biorxiv.org/content/10.1101/2022.11.28.518232)（bioRxiv）


### 5. ngless

- url: [https://github.com/ngless-toolkit/ngless](https://github.com/ngless-toolkit/ngless)
- repo: `ngless-toolkit/ngless`
- 说明: NGLess: NGS with less work
- GitHub: ★ 152；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [NG-meta-profiler: fast processing of metagenomes using NGLess, a domain-specific language.](https://doi.org/10.1186/s40168-019-0684-8)（Microbiome, 2019）
  - [NGLess: a domain-specific language for NGS processing.](https://doi.org/10.1093/gigascience/giz079)（GigaScience, 2019）


### 6. panacus

- url: [https://github.com/codialab/panacus](https://github.com/codialab/panacus)
- repo: `codialab/panacus`
- 说明: Panacus is a tool for computing statistics for GFA-formatted pangenome graphs
- GitHub: ★ 138；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Panacus: fast and exact pangenome growth and core size estimation](https://www.biorxiv.org/content/10.1101/2024.06.11.598418)（bioRxiv）
  - [Panacus: fast and exact pangenome growth and core size estimation.](https://doi.org/10.1093/bioinformatics/btae720)（Bioinformatics, 2024）


### 7. fastVEP

- url: [https://github.com/Huang-lab/fastVEP](https://github.com/Huang-lab/fastVEP)
- repo: `Huang-lab/fastVEP`
- 说明: fastVEP: High-performance Variant Effect Predictor in Rust
- GitHub: ★ 131；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [fastVEP: A Fast, Comprehensive Variant Effect Predictor Written in Rust](https://www.biorxiv.org/content/10.64898/2026.04.14.718452)（bioRxiv）
  - [fastVEP](https://www.biorxiv.org/content/)（bioRxiv）
  - [fastVEP](https://www.biorxiv.org/content/10.64898/2026.04.14.718452v1)（bioRxiv）


### 8. rustybam

- url: [https://github.com/vollgerlab/rustybam](https://github.com/vollgerlab/rustybam)
- repo: `vollgerlab/rustybam`
- 说明: bioinformatics toolkit in rust
- GitHub: ★ 107；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [rustybam:  a composable toolkit for alignment analysis and visualization with  SafFire](https://www.biorxiv.org/content/10.64898/2026.02.16.706142)（bioRxiv, 2026）


### 9. longcallR

- url: [https://github.com/huangnengCSU/longcallR](https://github.com/huangnengCSU/longcallR)
- repo: `huangnengCSU/longcallR`
- 说明: longcallR is a tool for SNP calling, haplotype phasing, and allele-specific analysis with long-read RNA-seq data.
- GitHub: ★ 96；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [SNP calling, haplotype phasing and allele-specific analysis with long RNA-seq reads](https://www.biorxiv.org/content/10.1101/2025.05.26.656191)（bioRxiv）


### 10. odon

- url: [https://github.com/alexcoulton/odon](https://github.com/alexcoulton/odon)
- repo: `alexcoulton/odon`
- 说明: Ultra-fast spatial proteomics OME-Zarr viewer built in Rust.
- GitHub: ★ 78；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Odon: an ultra-fast viewer for spatial proteomics.](https://doi.org/10.1093/bioinformatics/btag514)（Bioinformatics, 2026）
  - [Odon: An ultra-fast viewer for spatial proteomics](https://www.biorxiv.org/content/10.64898/2026.03.30.715233)（bioRxiv, 2026）


### 11. savont

- url: [https://github.com/bluenote-1577/savont](https://github.com/bluenote-1577/savont)
- repo: `bluenote-1577/savont`
- 说明: Amplicon sequencing variants from 16s ONT R10.4 / HiFi long reads
- GitHub: ★ 53；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Sensitive long-read amplicon sequence variant recovery with savont](https://www.biorxiv.org/content/10.64898/2026.05.26.727271)（bioRxiv）
  - [savont](https://www.biorxiv.org/content/)（bioRxiv）
  - [savont](https://www.biorxiv.org/content/10.64898/2026.05.26.727271v1)（bioRxiv）


### 12. RastQC

- url: [https://github.com/Huang-lab/RastQC](https://github.com/Huang-lab/RastQC)
- repo: `Huang-lab/RastQC`
- 说明: RastQC - Combining FastQC, MultiQC, longread QC functionality for high-throughput sequencing data, written in Rust
- GitHub: ★ 52；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [RastQC: High-Performance Sequencing Quality Control Written in Rust](https://www.biorxiv.org/content/10.64898/2026.03.31.715630)（bioRxiv）
  - [RastQC](https://www.biorxiv.org/content/)（bioRxiv）
  - [RastQC](https://www.biorxiv.org/content/10.64898/2026.03.31.715630v2)（bioRxiv）
  - [RastQC](https://www.biorxiv.org/content/10.64898/2026.03.31.71563)（bioRxiv）


### 13. SVTopo

- url: [https://github.com/PacificBiosciences/SVTopo](https://github.com/PacificBiosciences/SVTopo)
- repo: `PacificBiosciences/SVTopo`
- 说明: Complex structural variant visualization for HiFi sequencing data
- GitHub: ★ 51；last push ≥2026-07-31
- 满足: 论文、近1年维护、★≥50
- 文献:
  - [Complex structural variant visualization with SVTopo](https://www.biorxiv.org/content/10.1101/2025.04.16.649185)（bioRxiv）


### 14. cyto

- url: [https://github.com/ArcInstitute/cyto](https://github.com/ArcInstitute/cyto)
- repo: `ArcInstitute/cyto`
- 说明: A mapper for 10x-flex single cell sequencing reads with fixed abstract geometries
- GitHub: ★ 48；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [cyto: ultra high-throughput processing of 10x-flex single cell sequencing](https://www.biorxiv.org/content/10.64898/2026.01.21.700936)（bioRxiv）
  - [cyto](https://www.biorxiv.org/content/)（bioRxiv）
  - [cyto](https://www.biorxiv.org/content/10.64898/2026.01.21.700936v1):)（bioRxiv）


### 15. devider

- url: [https://github.com/bluenote-1577/devider](https://github.com/bluenote-1577/devider)
- repo: `bluenote-1577/devider`
- 说明: Dividing heterogeneous long-read sequencing into groups with de Bruijn graphs
- GitHub: ★ 48；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [devider: long-read reconstruction of many diverse haplotypes](https://www.biorxiv.org/content/10.1101/2024.11.05.621838)（bioRxiv）
  - [Long-read reconstruction of many diverse haplotypes with devider.](https://www.biorxiv.org/content/10.1101/gr.280510.125)（bioRxiv, 2025）


### 16. bronko

- url: [https://github.com/treangenlab/bronko](https://github.com/treangenlab/bronko)
- repo: `treangenlab/bronko`
- 说明: Ultra-rapid detection of viral variants directly from sequencing data
- GitHub: ★ 47；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [bronko: ultrafast, alignment-free detection of viral genome variation](https://www.biorxiv.org/content/10.64898/2025.12.01.691650)（bioRxiv）


### 17. locityper

- url: [https://github.com/tprodanov/locityper](https://github.com/tprodanov/locityper)
- repo: `tprodanov/locityper`
- 说明: Targeted genotyper for complex polymorphic genes
- GitHub: ★ 45；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Locityper: targeted genotyping of complex polymorphic genes](https://www.biorxiv.org/content/10.1101/2024.05.03.592358)（bioRxiv）
  - [Locityper enables targeted genotyping of complex polymorphic genes.](https://doi.org/10.1038/s41588-025-02362-4)（Nat Genet, 2025）


### 18. annotator

- url: [https://github.com/snijderlab/annotator](https://github.com/snijderlab/annotator)
- repo: `snijderlab/annotator`
- 说明: A simple tool to help you manually discover the depths of your (complex) spectra, one spectrum at a time.
- GitHub: ★ 42；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [A universal spectrum annotator for complex peptidoforms in mass spectrometry-based proteomics](https://www.biorxiv.org/content/10.1101/2025.01.18.633732)（bioRxiv, 2025）
  - [A Universal Spectrum Annotator for Complex Peptidoforms in Mass Spectrometry-Based Proteomics.](https://doi.org/10.1021/acs.analchem.5c02832)（Anal Chem, 2025）


### 19. gtfsort

- url: [https://github.com/alejandrogzi/gtfsort](https://github.com/alejandrogzi/gtfsort)
- repo: `alejandrogzi/gtfsort`
- 说明: a lexicographically-based GTF/GFF sorter
- GitHub: ★ 39；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [gtfsort: a tool to efficiently sort GTF files](https://www.biorxiv.org/content/10.1101/2023.10.21.563454)（bioRxiv）


### 20. JanusX

- url: [https://github.com/FJingxian/JanusX](https://github.com/FJingxian/JanusX)
- repo: `FJingxian/JanusX`
- 说明: A high-performance, ALL-in-ONE suite for quantitative genetics that unifies genome-wide association studies (GWAS) and genomic selection (GS).
- GitHub: ★ 37；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [JanusX: an integrated and high-performance platform for scalable genome-wide association studies and genomic selection](https://www.biorxiv.org/content/10.64898/2026.01.20.700366)（bioRxiv）
  - [JanusX](https://www.biorxiv.org/content/)（bioRxiv）


### 21. MAGmax

- url: [https://github.com/soedinglab/MAGmax](https://github.com/soedinglab/MAGmax)
- repo: `soedinglab/MAGmax`
- 说明: From dereplication to genome enrichment: Enhancing genome recovery across metagenomic samples using MAGmax
- GitHub: ★ 37；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Enhancing genome recovery across metagenomic samples using MAGmax.](https://doi.org/10.1093/bioinformatics/btaf538)（Bioinformatics, 2025）
  - [Enhancing genome recovery across metagenomic samples using MAGmax](https://www.biorxiv.org/content/10.1101/2025.05.28.656617)（bioRxiv, 2025）


### 22. termal

- url: [https://github.com/sib-swiss/termal](https://github.com/sib-swiss/termal)
- repo: `sib-swiss/termal`
- 说明: Terminal-based multiple sequence alignment (MSA) viewer designed for remote and HPC bioinformatics workflows.
- GitHub: ★ 33；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [termal: a terminal-based multiple sequence alignment viewer.](https://doi.org/10.1093/bioadv/vbaf208)（Bioinformatics Advances, 2025）


### 23. sketchlib.rust

- url: [https://github.com/bacpop/sketchlib.rust](https://github.com/bacpop/sketchlib.rust)
- repo: `bacpop/sketchlib.rust`
- 说明: Fast sequence distance estimates
- GitHub: ★ 31；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Rapid and Consistent Genome Clustering for Navigating Bacterial Diversity with Millions of MAGs and Isolates](https://www.biorxiv.org/content/10.64898/2025.12.30.695181)（bioRxiv）


### 24. xgt

- url: [https://github.com/Ebedthan/xgt](https://github.com/Ebedthan/xgt)
- repo: `Ebedthan/xgt`
- 说明: CLI tool for querying the Genome Taxonomy Database (GTDB). Batch search, genome cards, taxon lineages, and cross-release taxonomic comparison. JSON · CSV · TSV · stdin · parallel pagination · no runtime dependencies.
- GitHub: ★ 31；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [xgt: a command-line interface for the Genome Taxonomy Database with cross-release taxonomic comparison.](https://doi.org/10.1093/gigascience/giag086)（Gigascience, 2026）


### 25. STRdust

- url: [https://github.com/wdecoster/STRdust](https://github.com/wdecoster/STRdust)
- repo: `wdecoster/STRdust`
- 说明: Tandem repeat genotyping from long reads
- GitHub: ★ 27；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [A comprehensive assessment of tandem repeat genotyping methods for Nanopore long-read genomes](https://www.biorxiv.org/content/10.64898/2026.02.28.708646)（bioRxiv）
  - [STRdust](https://www.biorxiv.org/content/)（bioRxiv）
  - [STRdust](https://www.biorxiv.org/content/10.64898/2026.02.28.708646v1)（bioRxiv）


### 26. phylo2vec

- url: [https://github.com/sbhattlab/phylo2vec](https://github.com/sbhattlab/phylo2vec)
- repo: `sbhattlab/phylo2vec`
- 说明: phylo2vec: a library for vector-based phylogenetic tree manipulation
- GitHub: ★ 21；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [phylo2vec](https://doi.org/10.21105/joss.09040)（journal）
  - [Phylo2Vec: A Vector Representation for Binary Trees.](https://doi.org/10.1093/sysbio/syae030)（Syst Biol, 2025）


### 27. simd-sketch

- url: [https://github.com/RagnarGrootKoerkamp/simd-sketch](https://github.com/RagnarGrootKoerkamp/simd-sketch)
- repo: `RagnarGrootKoerkamp/simd-sketch`
- 说明: Compute bottom-s sketches and s-buckets sketches, using simd-minimizers crate.
- GitHub: ★ 21；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [SimdMinimizers: Computing random minimizers, fast](https://www.biorxiv.org/content/10.1101/2025.01.27.634998)（bioRxiv）
  - [simd-sketch](https://www.biorxiv.org/content/10.1101/2025.01.27.634998):)（bioRxiv）


### 28. ITSxRust

- url: [https://github.com/ayobi/ITSxRust](https://github.com/ayobi/ITSxRust)
- repo: `ayobi/ITSxRust`
- 说明: ITS region extraction for long-read amplicon sequencing
- GitHub: ★ 20；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [ITSxRust: ITS region extraction with partial-chain recovery and structured diagnostics for long-read amplicon sequencing](https://www.biorxiv.org/content/10.64898/2026.02.25.707950)（bioRxiv, 2026）


### 29. gbz-base

- url: [https://github.com/jltsiren/gbz-base](https://github.com/jltsiren/gbz-base)
- repo: `jltsiren/gbz-base`
- 说明: Pangenome file formats based on SQLite
- GitHub: ★ 16；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [GBZ-base and GAF-base: Indexed pangenome file formats](https://www.biorxiv.org/content/10.64898/2026.07.10.737775)（bioRxiv）


### 30. CycSim

- url: [https://github.com/BioEarthDigital/CycSim](https://github.com/BioEarthDigital/CycSim)
- repo: `BioEarthDigital/CycSim`
- 说明: A context-based long-read simulator
- GitHub: ★ 13；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Context-aware simulation enables systematic optimization of long-read mapping parameters.](https://doi.org/10.1093/gigascience/giag079)（GigaScience, 2026）


### 31. inquiSTR

- url: [https://github.com/wdecoster/inquiSTR](https://github.com/wdecoster/inquiSTR)
- repo: `wdecoster/inquiSTR`
- 说明: Genotyping of STRs with long reads
- GitHub: ★ 12；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Defining a tandem repeat catalog and variation clusters for genome-wide analyses and population databases](https://www.biorxiv.org/content/10.1101/2024.10.04.615514)（bioRxiv）
  - [inquiSTR: a toolkit for accurate and efficient population-scale tandem repeat genotyping and analysis](https://www.biorxiv.org/content/10.64898/2026.06.09.731080)（bioRxiv）
  - [inquiSTR](https://www.biorxiv.org/content/)（bioRxiv）


### 32. skiver

- url: [https://github.com/GZHoffie/skiver](https://github.com/GZHoffie/skiver)
- repo: `GZHoffie/skiver`
- 说明: Skiver: Reference-free quality control of (meta)genomic sequencing datasets using (k, v)-mer sketches
- GitHub: ★ 12；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Skiver: Alignment-free Estimation of Sequencing Error Rates and Spectra using (k, v)-mer Sketches](https://www.biorxiv.org/content/10.64898/2026.02.12.705514)（bioRxiv）
  - [skiver](https://www.biorxiv.org/content/)（bioRxiv）
  - [skiver](https://www.biorxiv.org/content/10.64898/2026.02.12.705514v2)（bioRxiv）


### 33. DartUniFrac

- url: [https://github.com/jianshu93/DartUniFrac](https://github.com/jianshu93/DartUniFrac)
- repo: `jianshu93/DartUniFrac`
- 说明: Approximate UniFrac via Weighted MinHash 🦀
- GitHub: ★ 11；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Megascale microbiome analysis with DartUniFrac.](https://doi.org/10.1038/s41587-026-03260-8)（Nat Biotechnol, 2026）


### 34. dna_parser

- url: [https://github.com/Mvila035/dna_parser](https://github.com/Mvila035/dna_parser)
- repo: `Mvila035/dna_parser`
- 说明: A Python module written in rust to encode DNA sequences for machine learning
- GitHub: ★ 11；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [dna-parser: a Python library written in Rust for fast encoding of DNA and RNA sequences](https://www.biorxiv.org/content/10.64898/2026.01.20.700656)（bioRxiv, 2026）


### 35. umgap

- url: [https://github.com/unipept/umgap](https://github.com/unipept/umgap)
- repo: `unipept/umgap`
- 说明: A taxonomic classifier for shotgun metagenomics reads
- GitHub: ★ 11；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [UMGAP: the Unipept MetaGenomics Analysis Pipeline.](https://doi.org/10.1186/s12864-022-08542-4)（BMC Genomics, 2022）


### 36. DeepChopper

- url: [https://github.com/ylab-hi/DeepChopper](https://github.com/ylab-hi/DeepChopper)
- repo: `ylab-hi/DeepChopper`
- 说明: Genomic Language Model Mitigates Chimera Artifacts in Nanopore Direct RNA Sequencing
- GitHub: ★ 9；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Genomic language model mitigates chimera artifacts in nanopore direct RNA sequencing.](https://doi.org/10.1038/s41467-026-68571-5)（Nat Commun, 2026）


### 37. fishnet

- url: [https://github.com/dietvin/fishnet](https://github.com/dietvin/fishnet)
- repo: `dietvin/fishnet`
- 说明: Fast and straightforward nanopore signal-to-sequence alignment
- GitHub: ★ 7；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [Fishnet simplifies and accelerates signal-to-sequence alignment in Nanopore sequencing](https://doi.org/10.21203/rs.3.rs-8345719/v1)（Research Square, 2026）


### 38. orthoSynAssign

- url: [https://github.com/stajichlab/orthoSynAssign](https://github.com/stajichlab/orthoSynAssign)
- repo: `stajichlab/orthoSynAssign`
- 说明: Python interface with a Rust synteny engine to refine orthogroups.
- GitHub: ★ 5；last push 2026-08-13
- 满足: 论文、近1年维护
- 文献:
  - [orthoSynAssign: refine orthogroups using synteny information](https://www.biorxiv.org/content/10.64898/2026.08.10.744007)（bioRxiv）
  - [orthoSynAssign](https://www.biorxiv.org/content/10.64898/2026.08.10.744007-blue)（bioRxiv）


### 39. eidolon

- url: [https://github.com/ncsa/eidolon](https://github.com/ncsa/eidolon)
- repo: `ncsa/eidolon`
- 说明: Eidolon is a Rust implementation of the next-gen sequencing toolkit (NEAT). Eidolon features expanded features, including the ability to model cancer genetics, complex variants, location aware variant placement, and allele dosage.
- GitHub: ★ 4；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [eidolon](https://doi.org/10.21105/joss.09056)（journal）


### 40. emits

- url: [https://github.com/ayobi/emits](https://github.com/ayobi/emits)
- repo: `ayobi/emits`
- 说明: Expectation-Maximization abundance estimation for fungal ITS communities from long-read sequencing
- GitHub: ★ 3；last push ≥2026-07-31
- 满足: 论文、近1年维护
- 文献:
  - [EMITS: expectation-maximization abundance estimation for fungal ITS communities from long-read sequencing](https://www.biorxiv.org/content/10.64898/2026.03.31.715662)（bioRxiv）


## 无文献（近 1 年维护或 ★≥50）

这些仓库没有检索到「介绍该工具」的预印本/期刊文，但本批均在 2026-07-31 之后有 push，因此靠第 2 条留下。★≥50 的另外标出。

| 工具 | 仓库 | ★ | last push | 满足 | 说明 |
| --- | --- | ---: | --- | --- | --- |
| [rosalind](https://github.com/logannye/rosalind) | `logannye/rosalind` | 278 | ≥2026-07-31 | 近1年维护、★≥50 | Deterministic, resource-governed per-locus genomics analyses with transactional artifacts, receipts, replay, Arrow, and reproducible sharding. |
| [sprocket](https://github.com/stjude-rust-labs/sprocket) | `stjude-rust-labs/sprocket` | 193 | ≥2026-07-31 | 近1年维护、★≥50 | A bioinformatics workflow engine built on top of the Workflow Description Language (WDL). |
| [molchanica](https://github.com/David-OConnor/molchanica) | `David-OConnor/molchanica` | 150 | ≥2026-07-31 | 近1年维护、★≥50 | Protein and molecule viewer, editor, simulator |
| [RustQC](https://github.com/seqeralabs/RustQC) | `seqeralabs/RustQC` | 122 | ≥2026-07-31 | 近1年维护、★≥50 | Fast genomics quality control tools for sequencing data, written in Rust. |
| [rust-lapper](https://github.com/sstadick/rust-lapper) | `sstadick/rust-lapper` | 71 | ≥2026-07-31 | 近1年维护、★≥50 | Rust implementation of a fast, easy, interval tree library nim-lapper |
| [crankshaft](https://github.com/stjude-rust-labs/crankshaft) | `stjude-rust-labs/crankshaft` | 56 | ≥2026-07-31 | 近1年维护、★≥50 | A Rust-based, headless workflow execution framework supporting local, cloud, and HPC. |
| [fgumi](https://github.com/fulcrumgenomics/fgumi) | `fulcrumgenomics/fgumi` | 52 | ≥2026-07-31 | 近1年维护、★≥50 | High-performance UMI tools for NGS data analysis |
| [gbwt-rs](https://github.com/jltsiren/gbwt-rs) | `jltsiren/gbwt-rs` | 46 | ≥2026-07-31 | 近1年维护 | GBZ file format for pangenome graphs |
| [gen](https://github.com/genhub-bio/gen) | `genhub-bio/gen` | 35 | ≥2026-07-31 | 近1年维护 | Git for genomes |
| [dynamite](https://github.com/raw-lab/dynamite) | `raw-lab/dynamite` | 33 | ≥2026-07-31 | 近1年维护 | One pure-Rust binary that finds genes in phages, viruses, giant viruses, crassphages, bacteria, archaea, eukaryotes — and raw sequencing reads. |
| [FasterFASTA](https://github.com/unum-science/FasterFASTA) | `unum-science/FasterFASTA` | 31 | ≥2026-07-31 | 近1年维护 | Faster FASTA and FASTQ file processing command line tool - parsing, sorting, deduplication, transliteration, filter, and stats |
| [microBioRust](https://github.com/LCrossman/microBioRust) | `LCrossman/microBioRust` | 31 | ≥2026-07-31 | 近1年维护 | Rust bioinformatics crate aimed at Microbial genomics |
| [seqair](https://github.com/Softleif/seqair) | `Softleif/seqair` | 28 | ≥2026-07-31 | 近1年维护 | BAM/SAM/CRAM/FASTA reader, pileup engine, BCF/BAM writing |
| [ferro-hgvs](https://github.com/fulcrumgenomics/ferro-hgvs) | `fulcrumgenomics/ferro-hgvs` | 27 | ≥2026-07-31 | 近1年维护 | A high-performance HGVS variant nomenclature parser and normalizer written in Rust |
| [fqless](https://github.com/openpaul/fqless) | `openpaul/fqless` | 26 | ≥2026-07-31 | 近1年维护 | less like viewer for fastq files |
| [ProForma](https://github.com/HUPO-PSI/ProForma) | `HUPO-PSI/ProForma` | 26 | ≥2026-07-31 | 近1年维护 | HUPO-PSI Standardized peptidoform notation |
| [oxo-call](https://github.com/Traitome/oxo-call) | `Traitome/oxo-call` | 23 | ≥2026-07-31 | 近1年维护 | Model-intelligent orchestration for CLI bioinformatics |
| [bed2gtf](https://github.com/alejandrogzi/bed2gtf) | `alejandrogzi/bed2gtf` | 21 | ≥2026-07-31 | 近1年维护 | high-performance BED-to-GTF converter written in Rust |
| [genotypst](https://github.com/apcamargo/genotypst) | `apcamargo/genotypst` | 20 | ≥2026-07-31 | 近1年维护 | genotypst: A bioinformatics Typst package for bioinformatics data analysis and visualization |
| [packed-seq](https://github.com/rust-seq/packed-seq) | `rust-seq/packed-seq` | 19 | ≥2026-07-31 | 近1年维护 | Bitpacked sequence trait and implementation |
| [spindalis](https://github.com/lignum-vitae/spindalis) | `lignum-vitae/spindalis` | 18 | ≥2026-07-31 | 近1年维护 | A bioinformatics-focused library for numerical modeling, optimisation, and simulation written in Rust |
| [snpick](https://github.com/PathoGenOmics-Lab/snpick) | `PathoGenOmics-Lab/snpick` | 17 | ≥2026-07-31 | 近1年维护 | snpick is a fast and memory-efficient Rust-based SNP extraction tool designed to handle large genomic alignments with minimal RAM usage and high-speed performance. |
| [sparrowhawk-asm](https://github.com/bacpop/sparrowhawk-asm) | `bacpop/sparrowhawk-asm` | 15 | ≥2026-07-31 | 近1年维护 | Short-read assembler for bacterial genomics based on a de Bruijn graph written in Rust 🦅 |
| [chelae](https://github.com/fulcrumgenomics/chelae) | `fulcrumgenomics/chelae` | 13 | ≥2026-07-31 | 近1年维护 | Fast, highly accurate, read-trimming for NGS data. |
| [irma-core](https://github.com/CDCgov/irma-core) | `CDCgov/irma-core` | 13 | ≥2026-07-31 | 近1年维护 | A tool to aid virus sequencing and accelerate IRMA. |
| [OpenTFRaw](https://github.com/Sigilweaver/OpenTFRaw) | `Sigilweaver/OpenTFRaw` | 13 | ≥2026-07-31 | 近1年维护 | Pure-Rust reader for Thermo Fisher .raw mass-spectrometry files, with Python bindings. |
| [COSMolKit](https://github.com/cosmol-studio/COSMolKit) | `cosmol-studio/COSMolKit` | 12 | ≥2026-07-31 | 近1年维护 | Pure-Rust cheminformatics toolkit and RDKit-compatible alternative for SMILES/SMARTS, SDF, fingerprints, substructure search, ETKDG, UFF/MMFF, and Python workflows. |
| [OpenMassSpec](https://github.com/Sigilweaver/OpenMassSpec) | `Sigilweaver/OpenMassSpec` | 10 | ≥2026-07-31 | 近1年维护 | High-performance Rust + Python readers for mass spectrometry / proteomics raw data. |
| [bgzf](https://github.com/fulcrumgenomics/bgzf) | `fulcrumgenomics/bgzf` | 9 | ≥2026-07-31 | 近1年维护 | BGZF compression library in Rust |
| [mako](https://github.com/fg-labs/mako) | `fg-labs/mako` | 9 | ≥2026-07-31 | 近1年维护 | Fast SAM/BAM sorter. |
| [arpeggia](https://github.com/y1zhou/arpeggia) | `y1zhou/arpeggia` | 8 | ≥2026-07-31 | 近1年维护 | Calculation of interatomic interactions in molecular structures |
| [zoe](https://github.com/CDCgov/zoe) | `CDCgov/zoe` | 8 | ≥2026-07-31 | 近1年维护 | Zoe provides both broad and highly specialized implementations for bioinformatics. In particular, we focus on common data formats and methods relevant for the sequencing of RNA viruses. |
| [haddock-restraints](https://github.com/haddocking/haddock-restraints) | `haddocking/haddock-restraints` | 7 | ≥2026-07-31 | 近1年维护 | Generate restraints to be used in HADDOCK |
| [nanoget-rs](https://github.com/wdecoster/nanoget-rs) | `wdecoster/nanoget-rs` | 7 | ≥2026-07-31 | 近1年维护 | Rust implementation of nanoget - fast extraction of nanopore sequencing metrics |
| [pandedup](https://github.com/RagnarGrootKoerkamp/pandedup) | `RagnarGrootKoerkamp/pandedup` | 7 | ≥2026-07-31 | 近1年维护 | Pandedup uses minimizers to build a quick & dirty k-mer spectrum (SPSS) from a .AGC pangenome. |
| [cfDNAlab](https://github.com/BesenbacherLab/cfDNAlab) | `BesenbacherLab/cfDNAlab` | 6 | ≥2026-07-31 | 近1年维护 | Ultra-fast command line tools for extracting bias-corrected fragmentation patterns from cell-free DNA |
| [holodeck](https://github.com/fg-labs/holodeck) | `fg-labs/holodeck` | 6 | ≥2026-07-31 | 近1年维护 | NGS read simulator |
| [rs_demultiplex](https://github.com/colindaven/rs_demultiplex) | `colindaven/rs_demultiplex` | 6 | ≥2026-07-31 | 近1年维护 | Simple but fast demultiplexing of FASTQ |
| [umi-tools-rs](https://github.com/vertti/umi-tools-rs) | `vertti/umi-tools-rs` | 6 | ≥2026-07-31 | 近1年维护 | A drop-in replacement for UMI-tools, written in Rust. Same flags, same output - 14-91x faster. |
| [chitin](https://github.com/chitin-dev/chitin) | `chitin-dev/chitin` | 5 | ≥2026-07-31 | 近1年维护 | 🦀🧪 chitin: a modern, agent-native computational chemistry and bioinformatics integrated development suite |
| [distree](https://github.com/PathoGenOmics-Lab/distree) | `PathoGenOmics-Lab/distree` | 5 | ≥2026-07-31 | 近1年维护 | Extracts a distance matrix from a phylogeny (parallel, low-memory) |
| [WisePulse](https://github.com/cbg-ethz/WisePulse) | `cbg-ethz/WisePulse` | 5 | ≥2026-07-31 | 近1年维护 | WISE Loculus with V-Pipe – Infra for start-to-end viral wastewater analysis. |
| [bwa-mem3-rs](https://github.com/fg-labs/bwa-mem3-rs) | `fg-labs/bwa-mem3-rs` | 4 | ≥2026-07-31 | 近1年维护 | Rust FFI crate for bwa-mem3 with packed-BAM output and caller-owned parallelism |
| [dais-ribosome](https://github.com/CDCgov/dais-ribosome) | `CDCgov/dais-ribosome` | 4 | ≥2026-07-31 | 近1年维护 | DAIS-ribosome annotates CDS and protein products for supported virus genomes into database-oriented output. |
| [genohype](https://github.com/broadinstitute/genohype) | `broadinstitute/genohype` | 4 | ≥2026-07-31 | 近1年维护 | Fast, memory-efficient toolkit for querying Hail tables and VCFs, exporting genomic data, and running distributed GCP jobs. |
| [minibwa-bindings](https://github.com/fg-labs/minibwa-bindings) | `fg-labs/minibwa-bindings` | 4 | ≥2026-07-31 | 近1年维护 | Rust and Python bindings for minibwa — Heng Li's lightweight bwa |
| [schnelLFMM](https://github.com/kdm9/schnelLFMM) | `kdm9/schnelLFMM` | 4 | ≥2026-07-31 | 近1年维护 | LFMM Accelerated: large-scale GEA and GWAS using out-of-core processing |
| [SpecLance](https://github.com/Sigilweaver/SpecLance) | `Sigilweaver/SpecLance` | 4 | ≥2026-07-31 | 近1年维护 | Columnar, memory-mapped mass spectrometry data store powered by Lance. |
| [trackcluster-rs](https://github.com/lrslab/trackcluster-rs) | `lrslab/trackcluster-rs` | 4 | ≥2026-07-31 | 近1年维护 |  |
| [trackclusterTU](https://github.com/lrslab/trackclusterTU) | `lrslab/trackclusterTU` | 4 | ≥2026-07-31 | 近1年维护 | Fast interval similarity and scalable clustering for bacterial transcript units (TUs) from mapped long reads. |
| [vita](https://github.com/vita-rs/vita) | `vita-rs/vita` | 4 | ≥2026-07-31 | 近1年维护 | Atomistic and molecular life sciences in Rust. |
| [aspartik](https://github.com/kaathewisegit/aspartik) | `kaathewisegit/aspartik` | 3 | ≥2026-07-31 | 近1年维护 | Computational biology toolkit |
| [kira-ls-aligner](https://github.com/ARyaskov/kira-ls-aligner) | `ARyaskov/kira-ls-aligner` | 3 | ≥2026-07-31 | 近1年维护 | Unified short- and long-read sequence aligner written in Rust 2024. It combines minimap2-style minimizers and chaining with BWA-MEM2-style exact-match anchoring and output semantics. The goal is drop-in compatibility with bwa-mem pipelines while supporting long reads efficiently. |
| [OpenWRaw](https://github.com/Sigilweaver/OpenWRaw) | `Sigilweaver/OpenWRaw` | 3 | ≥2026-07-31 | 近1年维护 | Pure-Rust reader for Waters MassLynx .raw mass-spectrometry directories, with Python bindings. |
| [PansimNuc](https://github.com/samhorsfield96/PansimNuc) | `samhorsfield96/PansimNuc` | 3 | ≥2026-07-31 | 近1年维护 | A nucleotide-level pangenome simulator. |
| [popqc](https://github.com/TNS-Schrauwen/popqc) | `TNS-Schrauwen/popqc` | 3 | ≥2026-07-31 | 近1年维护 | A fast and scalable tool for population-level quality control and QC outlier detection in large cohort genomics datasets. |
| [ruranges-core](https://github.com/pyranges/ruranges-core) | `pyranges/ruranges-core` | 3 | ≥2026-07-31 | 近1年维护 | Blazing fast genomics algorithms |
| [xenium-panel-convert](https://github.com/demhadais/xenium-panel-convert) | `demhadais/xenium-panel-convert` | 3 | ≥2026-07-31 | 近1年维护 | A command-line utility for converting files to the formats accepted by the 10x Genomics Xenium Panel Designer. |
| [bijux-genomics](https://github.com/bijux/bijux-genomics) | `bijux/bijux-genomics` | 2 | ≥2026-07-31 | 近1年维护 |  |
| [BioDex](https://github.com/Jakeelamb/BioDex) | `Jakeelamb/BioDex` | 2 | ≥2026-07-31 | 近1年维护 | Animal stat TUI widget |
| [GenoLance](https://github.com/Sigilweaver/GenoLance) | `Sigilweaver/GenoLance` | 2 | ≥2026-07-31 | 近1年维护 | Columnar, multi-sample bioinformatics data store for variants, ClinVar, and beyond. |
| [kira-bam](https://github.com/ARyaskov/kira-bam) | `ARyaskov/kira-bam` | 2 | ≥2026-07-31 | 近1年维护 | High-performance BAM/SAM toolkit written in Rust 2024. Drop-in samtools-compatible CLI |
| [OpenTimsTDF](https://github.com/Sigilweaver/OpenTimsTDF) | `Sigilweaver/OpenTimsTDF` | 2 | ≥2026-07-31 | 近1年维护 | Pure-Rust reader for Bruker timsTOF .tdf / .tdf_bin files, with Python bindings. |
| [whittle](https://github.com/erdikilic/whittle) | `erdikilic/whittle` | 2 | ≥2026-07-31 | 近1年维护 | Fast, tag-aware long-read (ONT/PacBio) trimmer for FASTQ and unaligned BAM. Rewrites the position-indexed tags through trimming and splitting: MM/ML/MN modification calls, per-base kinetics, and ONT signal. |
| [BamNado](https://github.com/alsmith151/BamNado) | `alsmith151/BamNado` | 1 | ≥2026-07-31 | 近1年维护 | High-performance BAM file processing for genomics — Rust core with Python bindings via PyO3. Parallel coverage/pileup, flexible read filtering (strand, MAPQ, fragment length, barcodes, tags), and signal normalisation for single-cell and bulk sequencing workflows. |
| [cbp](https://github.com/wang-q/cbp) | `wang-q/cbp` | 1 | ≥2026-07-31 | 近1年维护 | `cbp` - a Cross-platform Binary Package manager |
| [FastaGuard](https://github.com/ehsanestaji/FastaGuard) | `ehsanestaji/FastaGuard` | 1 | ≥2026-07-31 | 近1年维护 | FASTA preflight QC for assemblies: validate, triage, and produce pipeline-ready JSON/HTML reports before QUAST, BUSCO, BlobToolKit, or annotation. |
| [gatk-rs](https://github.com/SynapticFour/gatk-rs) | `SynapticFour/gatk-rs` | 1 | ≥2026-07-31 | 近1年维护 | Rust reimplementation of the GATK4 HaplotypeCaller germline short-variant spine (HC → CombineGVCFs → GenotypeGVCFs → hard-filter VariantFiltration). |
| [GenBankViz](https://github.com/linsalrob/GenBankViz) | `linsalrob/GenBankViz` | 1 | ≥2026-07-31 | 近1年维护 | A WASM genbank viewer to look through your sequences! |
| [hnsw-rs](https://github.com/ronakgh97/hnsw-rs) | `ronakgh97/hnsw-rs` | 1 | ≥2026-07-31 | 近1年维护 | Simplified Generic HNSW Implementation |
| [karyon](https://github.com/PathoGenOmics-Lab/karyon) | `PathoGenOmics-Lab/karyon` | 1 | ≥2026-07-31 | 近1年维护 | Genomic track plots for Rust: composable tracks over a shared coordinate axis, rendered to standalone SVG with zero dependencies |
| [kira-bio-tools](https://github.com/ARyaskov/kira-bio-tools) | `ARyaskov/kira-bio-tools` | 1 | ≥2026-07-31 | 近1年维护 | kira-bt is a high-performance toolkit for VCF/BCF processing with bcftools-oriented command-line compatibility. |
| [legume-rs](https://github.com/causalpathlab/legume-rs) | `causalpathlab/legume-rs` | 1 | ≥2026-07-31 | 近1年维护 | Library for Exploring Genomics Using Machine learning Essentials |
| [LibreGene](https://github.com/dl-li/LibreGene) | `dl-li/LibreGene` | 1 | ≥2026-07-31 | 近1年维护 | An open-source, agent-ready plasmid editor. |
| [OmicsOps](https://github.com/bai123350/OmicsOps) | `bai123350/OmicsOps` | 1 | ≥2026-07-31 | 近1年维护 | An autonomous bioinformatics agent that turns research plans into reproducible omics workflows, automatically retrieves data, executes analyses, troubleshoots errors, and delivers validated results with minimal user intervention. |
| [OpenMassSpecCore](https://github.com/Sigilweaver/OpenMassSpecCore) | `Sigilweaver/OpenMassSpecCore` | 1 | ≥2026-07-31 | 近1年维护 | Shared core types and traits for the OpenProteo proteomics reader family. |
| [pangenome-range](https://github.com/a-r-d/pangenome-range) | `a-r-d/pangenome-range` | 1 | ≥2026-07-31 | 近1年维护 | Utility for converting large GBZ files to a special format to read efficiently over the internet inside of a browser. |
| [proteon](https://github.com/theGreatHerrLebert/proteon) | `theGreatHerrLebert/proteon` | 1 | ≥2026-07-31 | 近1年维护 | Structural bioinformatics toolkit in Rust with Python bindings |
| [rastair](https://github.com/bsbludwig/rastair) | `bsbludwig/rastair` | 1 | ≥2026-07-31 | 近1年维护 | CLI for simultaneous detection of genetic variants and methylated positions from TAPS+ or 5-base sequencing data |
| [refget-rs](https://github.com/fg-labs/refget-rs) | `fg-labs/refget-rs` | 1 | ≥2026-07-31 | 近1年维护 | GA4GH refget Sequences v2.0.0 and Sequence Collections v1.0.0 — server, client, and CLI tools in Rust |
| [sage-plus](https://github.com/pgarrett-scripps/sage-plus) | `pgarrett-scripps/sage-plus` | 1 | ≥2026-07-31 | 近1年维护 | Experimental downstream Sage distribution under active development; not an official Sage release |
| [sam-subsampler](https://github.com/jiangyun-fun/sam-subsampler) | `jiangyun-fun/sam-subsampler` | 1 | ≥2026-07-31 | 近1年维护 | Two-pass BAM/CRAM subsampler that tags selected reads in place (per-reference reservoir sampling, qname-dedup bias fix) |
| [turbo-picard](https://github.com/dnncha/turbo-picard) | `dnncha/turbo-picard` | 1 | ≥2026-07-31 | 近1年维护 | Picard-compatible Rust tools for faster SAM/BAM/CRAM, VCF, duplicate-marking and sequencing-QC workflows. |
| [wetSpring](https://github.com/syntheticChemistry/wetSpring) | `syntheticChemistry/wetSpring` | 1 | ≥2026-07-31 | 近1年维护 | Pure Rust metagenomics, analytical chemistry, and mathematical biology — 1,750+ tests, 5,700+ validation checks. GPU-accelerated bioinformatics. AGPL-3.0 |
| [AnnoCAT](https://github.com/annocat-project/AnnoCAT) | `annocat-project/AnnoCAT` | 0 | ≥2026-07-31 | 近1年维护 | Local GRCh38 variant annotation and whole-genome result review with fastVEP, DuckDB, and Parquet. |
| [badclip](https://github.com/lh3/badclip) | `lh3/badclip` | 0 | ≥2026-07-31 | 近1年维护 | Extract clipped alignment |
| [bbnorm-rs](https://github.com/Jakeelamb/bbnorm-rs) | `Jakeelamb/bbnorm-rs` | 0 | ≥2026-07-31 | 近1年维护 | Rust port of BBTools BBNorm read-depth normalization |
| [Binx](https://github.com/alex-sandercock/Binx) | `alex-sandercock/Binx` | 0 | ≥2026-07-31 | 近1年维护 | Binx: A Rust-based CLI tool for polyploid and diploid genomic analysis |
| [bio-rust](https://github.com/DarthPapalo/bio-rust) | `DarthPapalo/bio-rust` | 0 | ≥2026-07-31 | 近1年维护 | Simple Rust library for bioinformatics |
| [bioforge](https://github.com/bioforgeAI/bioforge) | `bioforgeAI/bioforge` | 0 | ≥2026-07-31 | 近1年维护 | a python library for bioinformatics |
| [biox-rs](https://github.com/biox-dev/biox-rs) | `biox-dev/biox-rs` | 0 | ≥2026-07-31 | 近1年维护 | A Rust-based bioinformatics toolkit for data processing |
| [cistron](https://github.com/copyleftdev/cistron) | `copyleftdev/cistron` | 0 | ≥2026-07-31 | 近1年维护 | A convention-safe genome-variant kernel: normalization + VCF/HGVS/VRS boundaries + liftover, validated byte-for-byte against bcftools, vrs-python, and biocommons hgvs. |
| [combine-web](https://github.com/vrbouza/combine-web) | `vrbouza/combine-web` | 0 | ≥2026-07-31 | 近1年维护 | Combine in your browser! WebAssembly-based adaptation of physics statistical analyses developed using sparrowhawk as reference |
| [DISEQ](https://github.com/liquidambargenusbolbitis859/DISEQ) | `liquidambargenusbolbitis859/DISEQ` | 0 | ≥2026-07-31 | 近1年维护 | 🛰 Build a distributed message sequencer using zero knowledge consensus for reliable, honest data sequencing with improved efficiency. |
| [dnoise](https://github.com/pgarrett-scripps/dnoise) | `pgarrett-scripps/dnoise` | 0 | ≥2026-07-31 | 近1年维护 | Denoise Bruker timsTOF .d files by keeping ion-mobility streaks (Rust CLI + library + GUI) |
| [dragon](https://github.com/lcerdeira/dragon) | `lcerdeira/dragon` | 0 | ≥2026-07-31 | 近1年维护 | Dragon: a cloud-native aligner for surveillance-scale microbial genomics |
| [earl](https://github.com/Chandrikakt/earl) | `Chandrikakt/earl` | 0 | ≥2026-07-31 | 近1年维护 | Provides a safe command-line interface to manage AI agents using HTTP, GraphQL, gRPC, Bash, and SQL protocols across platforms. |
| [fastcover](https://github.com/jianshu93/fastcover) | `jianshu93/fastcover` | 0 | ≥2026-07-31 | 近1年维护 | Metagenomic coverage and diversity estimation for long reads, fast |
| [Gipfelkreuzer](https://github.com/at-robins/Gipfelkreuzer) | `at-robins/Gipfelkreuzer` | 0 | ≥2026-07-31 | 近1年维护 | Creates consensus peaks from raw peaks called on CHIP or ATAC data. |
| [gnomad-lr](https://github.com/broadinstitute/gnomad-lr) | `broadinstitute/gnomad-lr` | 0 | ≥2026-07-31 | 近1年维护 | gnomAD Long Read VCF→ClickHouse loading pipeline |
| [HiFiRe3](https://github.com/wilsontelab/HiFiRe3) | `wilsontelab/HiFiRe3` | 0 | ≥2026-07-31 | 近1年维护 | Pipelines and apps for analyzing sequencing data generated by HiFiRe3 restriction enzyme reduced representation long read methods |
| [HmnRandomRead](https://github.com/guillaume-gricourt/HmnRandomRead) | `guillaume-gricourt/HmnRandomRead` | 0 | ≥2026-07-31 | 近1年维护 | A sequence-read simulator program for NGS |
| [hox-core](https://github.com/Jakeelamb/hox-core) | `Jakeelamb/hox-core` | 0 | ≥2026-07-31 | 近1年维护 | Rust Bio OS workspace: mmap I/O, indexes, alignment, columnar outputs (HoxBio-style) |
| [humas_hmmer](https://github.com/aglabx/humas_hmmer) | `aglabx/humas_hmmer` | 0 | ≥2026-07-31 | 近1年维护 | Parallel alpha-satellite HOR/SF annotation: single-binary Rust port of the HumAS-HMMER pipeline |
| [isotools](https://github.com/alejandrogzi/isotools) | `alejandrogzi/isotools` | 0 | ≥2026-07-31 | 近1年维护 | tools for long-read transcriptomics |
| [linxira-bio-sdk](https://github.com/Linxira-OS/linxira-bio-sdk) | `Linxira-OS/linxira-bio-sdk` | 0 | ≥2026-07-31 | 近1年维护 | Local-first bioinformatics SDK with a native GUI, CLI, and agent skills. |
| [mercure](https://github.com/SteampunkIslande/mercure) | `SteampunkIslande/mercure` | 0 | ≥2026-07-31 | 近1年维护 | A simple webapp designed to automate sequencing analysis using slurm, snakemake and singularity. |
| [mzLibRust](https://github.com/smith-chem-wisc/mzLibRust) | `smith-chem-wisc/mzLibRust` | 0 | ≥2026-07-31 | 近1年维护 | mzLib for Rust — mass spectrometry and proteomics, over the same language-neutral bridge as pyMzLib. |
| [Nucle-OS](https://github.com/VyomKulshrestha/Nucle-OS) | `VyomKulshrestha/Nucle-OS` | 0 | ≥2026-07-31 | 近1年维护 | A software-defined DNA storage operating system — encode, protect, and retrieve real files as synthetic DNA, with its own domain-specific language (NucleScript), a CLI/VFS runtime, error correction, encryption, and Prometheus metrics. |
| [open-king](https://github.com/Broccolito/open-king) | `Broccolito/open-king` | 0 | ≥2026-07-31 | 近1年维护 | Rust Reimplementation of KING (Kinship-based INference for GWAS) |
| [pan2met-rs](https://github.com/labgem/pan2met-rs) | `labgem/pan2met-rs` | 0 | ≥2026-07-31 | 近1年维护 | pan2met: predict metabolic network at pangenome scale, an alternative development in rust |
| [percolator-rs](https://github.com/AndrejRumenovski/percolator-rs) | `AndrejRumenovski/percolator-rs` | 0 | ≥2026-07-31 | 近1年维护 | A high-performance Rust reimplementation of Percolator for computational proteomics. Uses semi-supervised SVM training, cross-validation, and target-decoy FDR estimation for fast, memory-efficient PSM rescoring. Designed for scalable mass spectrometry workflows with reproducible results and up to 23× faster performance than C++. |
| [pg4findr](https://github.com/artorias111/pg4findr) | `artorias111/pg4findr` | 0 | ≥2026-07-31 | 近1年维护 | Search for G-quadruplex motifs in sequencing reads and genome assemblies. |
| [pggname](https://github.com/jltsiren/pggname) | `jltsiren/pggname` | 0 | ≥2026-07-31 | 近1年维护 | Pangenome graph naming based on hashing in a canonical order |
| [PhageFilter](https://github.com/Dreycey/PhageFilter) | `Dreycey/PhageFilter` | 0 | ≥2026-07-31 | 近1年维护 | PhageFilter uses a Sequence Bloom Tree (SBT) to filter bacteriophage reads from metagenomic files. |
| [phorge](https://github.com/andrewbudge/phorge) | `andrewbudge/phorge` | 0 | ≥2026-07-31 | 近1年维护 | A  CLI toolkit for phylogenetic analysis |
| [proxide](https://github.com/maraxen/proxide) | `maraxen/proxide` | 0 | ≥2026-07-31 | 近1年维护 | High-performance protein I/O and physics bridging for JAX, with a Rust backend for structure parsing and force-field parameterization |
| [qbix](https://github.com/kojix2/qbix) | `kojix2/qbix` | 0 | ≥2026-07-31 | 近1年维护 | Retrieves BAM records by read name |
| [rammap-web](https://github.com/vrbouza/rammap-web) | `vrbouza/rammap-web` | 0 | ≥2026-07-31 | 近1年维护 | Map reads in your browser! WebAssembly-based web platform of rammap-rs using sparrowhawk as reference |
| [realtime-core](https://github.com/abrahamahn/realtime-core) | `abrahamahn/realtime-core` | 0 | ≥2026-07-31 | 近1年维护 | Transport-neutral realtime subscriptions, sequencing, recovery, and command receipts |
| [rsomics-metagenomics](https://github.com/omics-rust/rsomics-metagenomics) | `omics-rust/rsomics-metagenomics` | 0 | ≥2026-07-31 | 近1年维护 | Abundance-aware amplicon processing workflows |
| [rsomics-methyl](https://github.com/omics-rust/rsomics-methyl) | `omics-rust/rsomics-methyl` | 0 | ≥2026-07-31 | 近1年维护 | Bisulfite-sequencing methylation extraction and bias QC in Rust |
| [rsomics-table](https://github.com/omics-rust/rsomics-table) | `omics-rust/rsomics-table` | 0 | ≥2026-07-31 | 近1年维护 | High-performance CSV and TSV workflows for bioinformatics |
| [rust-yara](https://github.com/fg-labs/rust-yara) | `fg-labs/rust-yara` | 0 | ≥2026-07-31 | 近1年维护 | Safe Rust FFI bindings for the YARA read mapper and indexer (SeqAn2) |
| [SAMRust](https://github.com/Caizhaohui/SAMRust) | `Caizhaohui/SAMRust` | 0 | ≥2026-07-31 | 近1年维护 | SAMRust is a Rust-native, multi-threaded, pysam-compatible HTS processing library optimized for large-scale sequencing analysis on Linux/HPC systems. |
| [segment](https://github.com/BiocomputeLab/segment) | `BiocomputeLab/segment` | 0 | ≥2026-07-31 | 近1年维护 | Flexible segment based analysis of sequencing data |
| [seqgc](https://github.com/ad3002/seqgc) | `ad3002/seqgc` | 0 | ≥2026-07-31 | 近1年维护 | A garbage collector for sequencing data: compresses raw FASTQ, replaces re-downloadable NCBI/SRA copies with verified restore scripts, and collapses duplicates. Plan/apply, never deletes before reading the replacement back. |
| [SeqQC](https://github.com/saikatbib/SeqQC) | `saikatbib/SeqQC` | 0 | ≥2026-07-31 | 近1年维护 | Local-first sequencing quality-control studio for WGS, WES, and bulk RNA-seq (engineering preview). |
| [somite](https://github.com/Jakeelamb/somite) | `Jakeelamb/somite` | 0 | ≥2026-07-31 | 近1年维护 | Local-first visual builder for reproducible bioinformatics workflows. |
| [sparrowhawk-graph](https://github.com/bacpop/sparrowhawk-graph) | `bacpop/sparrowhawk-graph` | 0 | ≥2026-07-31 | 近1年维护 | Crate to work with de Bruijn graphs on Rust, using petgraph |
| [tasmanian-mismatch](https://github.com/nebiolabs/tasmanian-mismatch) | `nebiolabs/tasmanian-mismatch` | 0 | ≥2026-07-31 | 近1年维护 | Analysis of artifacts in high throughput sequencing data from genomic DNA. |

## 未列入

- 非 GitHub Rust 生信分析工具：FrustrAI-Seq、pe-uncert、biomcp、wisp-science、helix、audio-gen、StringWars 等。
- 文献检索里的使用型引用（文章只用了该工具，标题未介绍该工具）不计入第 1 条。
