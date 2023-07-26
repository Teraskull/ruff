<a name="v0.0.280"></a>
# [v0.0.280](https://github.com/astral-sh/ruff/releases/tag/v0.0.280) - 23 Jul 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.280 -->

## What's Changed
### Bug Fixes
* Avoid collapsing `elif` and `else` branches during import sorting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5964

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.279...v0.0.280

[Changes][v0.0.280]


<a name="v0.0.279"></a>
# [v0.0.279](https://github.com/astral-sh/ruff/releases/tag/v0.0.279) - 21 Jul 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.279 -->

## What's Changed
### Rules
* [`flake8-pyi`] Implement flake8-pyi's PYI026 by [@LaBatata101](https://github.com/LaBatata101) in https://github.com/astral-sh/ruff/pull/5844
* [`flake8-pyi`] Implement flake8-pyi's `PYI017` by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/5895
* [`flake8-pyi`] Implement flake8-pyi's `PYI036` by [@density](https://github.com/density) in https://github.com/astral-sh/ruff/pull/5668
* [`flake8-pyi`] Implement flake8-pyi's `PYI041` by [@density](https://github.com/density) in https://github.com/astral-sh/ruff/pull/5722
* [`flake8-use-pathlib`] Implement `os-path-getsize` and `os-path-get(a|m|c)-time` (`PTH202-205`) by [@sbrugman](https://github.com/sbrugman) in https://github.com/astral-sh/ruff/pull/5835
* [`flake8-use-pathlib`] Implement `path-constructor-default-argument` (`PTH201`) by [@sbrugman](https://github.com/sbrugman) in https://github.com/astral-sh/ruff/pull/5833
* [`pandas-vet`] Implement constant series rule (`PD101`) by [@sbrugman](https://github.com/sbrugman) in https://github.com/astral-sh/ruff/pull/5802
* [`pylint`] Implement Pylint's `consider-using-in` (`PLR1714`) by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5193

### Rule changes
* [`flake8-annotations`] Check for `Any` in other types for `ANN401` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5601
* [`flake8-bugbear`] Add autofix for B004 by [@density](https://github.com/density) in https://github.com/astral-sh/ruff/pull/5788
* [`flake8-bugbear`] Remove `B904`'s lowercase exemption by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5751
* [`flake8-use-pathlib`] extend PTH118 with `os.sep` by [@sbrugman](https://github.com/sbrugman) in https://github.com/astral-sh/ruff/pull/5935
* [`pyupgrade`] Expand scope of `quoted-annotation` rule (`UP037`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5766
* [`pyupgrade`] Extend PEP 604 rewrites to support some quoted annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5725
* [`ruff`] Expand `RUF015` to include all expression types by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5767

### Bug Fixes
* Consider single element subscript expr for implicit optional by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5717
* Ignore `Enum`-and-`str` subclasses for slots enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5749
* Avoid removing raw strings in comparison fixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5755
* Fix nested calls to `sorted` with differing arguments by [@density](https://github.com/density) in https://github.com/astral-sh/ruff/pull/5761
* Use unused variable detection to power `incorrect-dict-iterator` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5763
* Include alias when formatting import-from structs by [@guillaumeLepape](https://github.com/guillaumeLepape) in https://github.com/astral-sh/ruff/pull/5786
* Make `lint_only` aware of the source kind by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5876
* Restore `redefined-while-unused` violations in classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5926
* Flatten nested tuples when fixing UP007 violations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5724
* Ignore Jupyter Notebooks for `--add-noqa` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5727
* Avoid checking `EXE001` and `EXE002` on WSL by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5735
* Properly group assignment targets by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/astral-sh/ruff/pull/5728
* Avoid stack overflow for non-BitOr binary types by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5743
* Move function visit out of `Expr::Call` branches by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5772
* [B006] Add bytes to immutable types by [@harupy](https://github.com/harupy) in https://github.com/astral-sh/ruff/pull/5776
* Format `SetComp` by [@lkh42t](https://github.com/lkh42t) in https://github.com/astral-sh/ruff/pull/5774
* Gate `runtime-import-in-type-checking-block` (`TCH004`) behind enabled flag by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5789
* perf: only compute start offset for overlong lines by [@sbrugman](https://github.com/sbrugman) in https://github.com/astral-sh/ruff/pull/5811
* Change `pandas-use-of-dot-read-table` rule to emit only when `read_table` is used on CSV data by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5807
* Do not fix `NamedTuple` calls containing both a list of fields and keywords by [@harupy](https://github.com/harupy) in https://github.com/astral-sh/ruff/pull/5799
* Ignore directories when collecting files to lint by [@harupy](https://github.com/harupy) in https://github.com/astral-sh/ruff/pull/5775
* Add filename to `noqa` warnings by [@sobolevn](https://github.com/sobolevn) in https://github.com/astral-sh/ruff/pull/5856
* Handle io errors gracefully by [@konstin](https://github.com/konstin) in https://github.com/astral-sh/ruff/pull/5611
* Allow `respect_gitignore` when not in a git repo by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5937

## New Contributors
* [@eggplants](https://github.com/eggplants) made their first contribution in https://github.com/astral-sh/ruff/pull/5741
* [@guillaumeLepape](https://github.com/guillaumeLepape) made their first contribution in https://github.com/astral-sh/ruff/pull/5786
* [@odiseo0](https://github.com/odiseo0) made their first contribution in https://github.com/astral-sh/ruff/pull/5888
* [@DavidCain](https://github.com/DavidCain) made their first contribution in https://github.com/astral-sh/ruff/pull/5889
* [@LaBatata101](https://github.com/LaBatata101) made their first contribution in https://github.com/astral-sh/ruff/pull/5844

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.278...v0.0.279

[Changes][v0.0.279]


<a name="v0.0.278"></a>
# [v0.0.278](https://github.com/astral-sh/ruff/releases/tag/v0.0.278) - 12 Jul 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

See the [release blog post](https://astral.sh/blog/ruff-v0.0.278) for more, including detailed descriptions of any newly added rules.

## What's Changed

### Rules
* [`pylint`] Implement `typevar-bivariance` (`PLC0131`)  by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5517
* [`flake8-pyi`] Implement  `unnecessary-literal-union` (`PYI030`) by [@zanieb](https://github.com/zanieb) in https://github.com/astral-sh/ruff/pull/5570
* [`pylint`] Implement `type-name-incorrect-variance` (`PLC0105`)  by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5651
* [`ruff`] Implement `unnecessary-list-allocation-for-first-element` (`RUF015`) by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/astral-sh/ruff/pull/5549
* [`flake8-bugbear`] Implement `re-sub-positional-args` (`B034`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5669
* [`ruff`] Implement `invalid-index-type` (`RUF016`) by [@zanieb](https://github.com/zanieb) in https://github.com/astral-sh/ruff/pull/5602

### Settings
* [`isort`] Add `--case-sensitive` flag by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/5539
* [`isort`] Support globbing in `isort` options by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5473

### Bug Fixes
* Support autofix for some multiline `str.format` calls by [@harupy](https://github.com/harupy) in https://github.com/astral-sh/ruff/pull/5638
* Avoid triggering `unnecessary-map` (`C417`) for late-bound lambdas by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5520
* Avoid triggering DTZ001-006 when using `.astimezone()` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5524
* Enable attribute lookups via semantic model by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5536
* Avoid syntax errors when rewriting str(dict) in f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5538
* Differentiate between runtime and typing-time annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5575
* Only run pyproject.toml lint rules when enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5578
* Refactor isort directive skips to use iterators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5623
* Allow descriptor instantiations in dataclass fields by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5537
* Refactor `noqa` directive parsing away from regex-based implementation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5554
* Emit warnings for invalid `# noqa` directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5571
* Support individual codes on `# flake8: noqa` directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5618
* Add `tkinter` import convention by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5626
* Avoid `PERF401` if conditional depends on list var by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5603
* Fix typo in complex-if-statement-in-stub message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5635
* Make TRY301 trigger only if a `raise` throws a caught exception  by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/astral-sh/ruff/pull/5455
* Skip flake8-future-annotations checks in stub files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5652
* Always allow PEP 585 and PEP 604 rewrites in stub files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5653
* Add support for `Union` declarations without `|` to PYI016 by [@zanieb](https://github.com/zanieb) in https://github.com/astral-sh/ruff/pull/5598
* Ignore `_name_` and `_value_` accesses in `flake8-self` rules by [@monosans](https://github.com/monosans) in https://github.com/astral-sh/ruff/pull/5663
* Refactor `repeated_keys()` to use `ComparableExpr` by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/5696

## New Contributors
* [@karosis88](https://github.com/karosis88) made their first contribution in https://github.com/astral-sh/ruff/pull/5560
* [@petermattia](https://github.com/petermattia) made their first contribution in https://github.com/astral-sh/ruff/pull/5579
* [@DimitriPapadopoulos](https://github.com/DimitriPapadopoulos) made their first contribution in https://github.com/astral-sh/ruff/pull/5607

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.277...v0.0.278

[Changes][v0.0.278]


<a name="v0.0.277"></a>
# [v0.0.277](https://github.com/astral-sh/ruff/releases/tag/v0.0.277) - 06 Jul 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.277 -->

## What's Changed

### Breaking Changes
* Add .ipynb_checkpoints, .pyenv, .pytest_cache, and .vscode to default excludes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5513

### Rules
* [`pylint`] Implement Pylint `typevar-name-mismatch` (`C0132`) by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5501

### Settings
* Add `ruff rule --all` subcommand (with JSON output) by [@akx](https://github.com/akx) in https://github.com/astral-sh/ruff/pull/5059

### Bug Fixes
* Fix eval detection for suspicious-eval-usage by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5506
* Avoid PERF rules for iteration-dependent assignments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5508
* Avoid returning first-match for rule prefixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5511

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.276...v0.0.277

[Changes][v0.0.277]


<a name="v0.0.276"></a>
# [v0.0.276](https://github.com/astral-sh/ruff/releases/tag/v0.0.276) - 03 Jul 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.276 -->

See the [release blog post](https://astral.sh/blog/ruff-v0.0.276) for more, including detailed descriptions of any newly added rules.

## What's Changed

Highlights include: experimental support for linting Jupyter Notebooks.

To opt-in to linting Jupyter Notebook files, add the `*.ipynb` pattern to your [`include`](settings.md#include)
setting, like so:

```toml
[tool.ruff]
# Allow Ruff to discover `*.ipynb` files.
include = ["*.py", "*.pyi", "**/pyproject.toml", "*.ipynb"]
```

This will prompt Ruff to discover Jupyter Notebook files in any specified directories, and lint them
accordingly.

Jupyter Notebook support is currently opt-in and experimental. We'd love your help testing it out.
Have feedback? Run into issues? [Let us know!](https://github.com/astral-sh/ruff/issues/new)

### New Rules

* [`flake8-pyi`] Implement `PYI002`, `PYI003`, `PYI004`, `PYI005` by [@density](https://github.com/density) in https://github.com/astral-sh/ruff/pull/5457
* [`numpy`] Implement `numpy-deprecated-function` (`NPY003`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5468
* [`perflint`] Implement `unnecessary-list-cast` (`PERF101`) by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/5121
* [`perflint`] Implement `try-except-in-loop` (`PERF203`) by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/astral-sh/ruff/pull/5166
* [`perflint`] Implement `manual-list-comprehension` (`PERF401`) and `manual-list-copy` (`PERF402`) rules by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/5298
* [`pylint`] Implement Pylint `single-string-used-for-slots` (`C0205`) as `single-string-slots` (`PLC0205`) by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5399

### Jupyter
* Experimental release for Jupyter notebook integration by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5363
* Enable --watch for Jupyter notebooks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5394
* Consider Jupyter index for code frames (`--show-source`) by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5402
* fixup! Consider Jupyter index for code frames (`--show-source`) ([#5402](https://github.com/astral-sh/ruff/issues/5402)) by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5414

### Settings
* [`pyupgrade`] Restore the `keep-runtime-typing` setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5470
* Add `PythonVersion::Py312` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5316

### Bug Fixes
* Support `pydantic.BaseSettings` in `mutable-class-default` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5312
* Allow `__slots__` assignments in `mutable-class-default` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5314
* Avoid syntax errors when removing f-string prefixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5319
* Ignore unpacking in `iteration-over-set` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5392
* Replace same length equal line with dash line in D407 by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5383
* Exclude docstrings from PYI053 by [@intgr](https://github.com/intgr) in https://github.com/astral-sh/ruff/pull/5405
* Use "manual" fixability for E731 in shadowed context by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5430
* Detect consecutive, non-newline-delimited NumPy sections by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5467
* Fix `unnecessary-encode-utf8` to fix `encode` on parenthesized strings correctly by [@harupy](https://github.com/harupy) in https://github.com/astral-sh/ruff/pull/5478
* Allow `Final` assignments in stubs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5490
* Respect `abc` decorators when classifying function types by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5315
* Allow `@Author` format for "Missing Author" rule in `flake8-todos` by [@mayrholu](https://github.com/mayrholu) in https://github.com/astral-sh/ruff/pull/4903
* Ignore type aliases for RUF013 by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5344
* Change W605 autofix to use raw strings if possible by [@hauntsaninja](https://github.com/hauntsaninja) in https://github.com/astral-sh/ruff/pull/5352
* Add space when migrating to raw string by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5358
* Update the `invalid-escape-sequence` rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5359
* Include BaseException in B017 rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5466
* [`flake8-django`] Skip duplicate violations in `DJ012` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5469

## New Contributors
* [@mayrholu](https://github.com/mayrholu) made their first contribution in https://github.com/astral-sh/ruff/pull/4903
* [@hauntsaninja](https://github.com/hauntsaninja) made their first contribution in https://github.com/astral-sh/ruff/pull/5352
* [@ethunk](https://github.com/ethunk) made their first contribution in https://github.com/astral-sh/ruff/pull/5397
* [@LouisDISPA](https://github.com/LouisDISPA) made their first contribution in https://github.com/astral-sh/ruff/pull/5475

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.275...v0.0.276

[Changes][v0.0.276]


<a name="v0.0.275"></a>
# [v0.0.275](https://github.com/astral-sh/ruff/releases/tag/v0.0.275) - 22 Jun 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.275 -->

## What's Changed

Highlights include a 7-10x decrease in Ruff's cache size.

### Rules
* Add support for top-level quoted annotations in RUF013 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5235
* Add support for nested quoted annotations in RUF013 by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5254
* Move `compare-to-empty-string` (`PLC1901`) to nursery by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5264
* Ignore Pydantic classes when evaluating `mutable-class-default` (`RUF012`)  by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5273
* Allow `typing.Final` for `mutable-class-default annotations` (`RUF012`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5274
* Modify `deprecated-import` (`UP035`) to prefer `typing_extensions` in some versions by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5291

### Bug Fixes
* Restore existing bindings when unbinding caught exceptions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5256
* Avoid including nursery rules in linter-level selectors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5268

## New Contributors
* [@jgberry](https://github.com/jgberry) made their first contribution in https://github.com/astral-sh/ruff/pull/5221

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.274...v0.0.275

[Changes][v0.0.275]


<a name="v0.0.274"></a>
# [v0.0.274](https://github.com/astral-sh/ruff/releases/tag/v0.0.274) - 21 Jun 2023

<!-- Release notes generated using configuration in .github/release.yml at v0.0.274 -->

## What's Changed

Follow-up release to `v0.0.273` to fix a panic in cache accesses.

### Bug Fixes
* Use package roots rather than package members for cache initialization by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5233
* Avoid `.unwrap()` on cache access by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5229
* Revert change to `RUF010` to remove unnecessary `str` calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5232
* Avoid erroneous RUF013 violations for quoted annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5234

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.273...v0.0.274

[Changes][v0.0.274]


<a name="v0.0.273"></a>
# [v0.0.273](https://github.com/astral-sh/ruff/releases/tag/v0.0.273) - 20 Jun 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

Highlights include:

- Autofix capabilities for rules like `flake8-import-conventions`, which require symbol renames across a file.
- Significant decrease in Ruff's cache size (e.g., a ~50% decrease for FastAPI).
- Dozens and dozens of bug fixes + performance improvements.

### Rules

* [`copyright`] Implement copyright notice detection by [@Ryang20718](https://github.com/Ryang20718) in https://github.com/astral-sh/ruff/pull/4701
* [`flake8-datetimez`] Enable UTC-import for `datetime-utc-alias` fix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5100
* [`flake8-implicit-str-concat`] Add autofix for `ISC001` by [@tkukushkin](https://github.com/tkukushkin) in https://github.com/astral-sh/ruff/pull/4853
* [`flake8-import-conventions`] Enable autofix for unconventional imports rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5152
* [`flake8-pyi`] Add autofix for `Set`-to-`AbstractSet` rewrite using reference tracking by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5074
* [`flake8-pyi`] Implement PYI044 by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5021
* [`flake8-return`] Extend revised `RET504` implementation to `with` statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4998
* [`flake8-return`] Implement autofix for revised `RET504` rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4999
* [`flake8-return`] Refactor `RET504` to only enforce assignment-then-return pattern by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4997
* [`flake8-slots`] Add plugin, add `SLOT000`, `SLOT001` and `SLOT002` by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/4909
* [`perflint`] Add `perflint` plugin, add first rule `PERF102` by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/astral-sh/ruff/pull/4821
* [`pylint`] Add Pylint rule `comparison-with-itself` (`R0124`) by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/4957
* [`pyupgrade`] Add a rule to remove unnecessary parentheses in class definitions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5032
* [`ruff`] Add a rule for static keys in dict comprehensions  by [@rodjunger](https://github.com/rodjunger) in https://github.com/astral-sh/ruff/pull/4929
* [`ruff`] Add rule to disallow implicit optional with autofix by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/4831
* [`ruff`] Expand RUF008 to all classes, but to a new code (RUF012) by [@adampauls](https://github.com/adampauls) in https://github.com/astral-sh/ruff/pull/4390
* [`ruff`] Remove unannotated attributes from RUF008 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5049
* [`ruff`] Upgrade explicit-type-conversion rule (`RUF010`) to remove unnecessary `str` calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4971

### Settings

* Option (`-o`/`--output-file`) to write output to a file by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/4950
* Add JSON Lines (NDJSON) message serialization by [@akx](https://github.com/akx) in https://github.com/astral-sh/ruff/pull/5048
* Add rule documentation URL to JSON output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5187

### Caching

* Only use a single cache file per Python package by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5117
* Open cache files in parallel by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5120

### Jupyter

* Add support for auto-fix in Jupyter notebooks by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/4665
* Add roundtrip support for Jupyter notebook by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5028

### Bug Fixes

* Handle decorators in class-parenthesis-modifying rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5034
* Allow re-assignments to `__all__` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4967
* Handled dict and set inside f-string ([#4249](https://github.com/astral-sh/ruff/issues/4249)) by [@DavideCanton](https://github.com/DavideCanton) in https://github.com/astral-sh/ruff/pull/4563
* Support concatenated string key removals by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4976
* Respect 'is not' operators split across newlines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4977
* Parenthesize expressions prior to lexing in F632 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5001
* Ignore pyproject.toml for adding noqa directives by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/astral-sh/ruff/pull/5013
* Support 'reason' argument to `pytest.fail` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5040
* Allow `async with` in `redefined-loop-name` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5125
* Skip `DJ008` enforcement in stub files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5139
* Detect continuations at start-of-file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5173
* Fix allowed-ellipsis detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5174
* Remove continuations before trailing semicolons by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5199
* Support parenthesized expressions when splitting compound assertions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5219
* Use phf for confusables to reduce llvm lines by [@konstin](https://github.com/konstin) in https://github.com/astral-sh/ruff/pull/4926
* Allow private accesses within special dunder methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4968
* Support concatenated literals in format-literals by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/4974
* Fix line numbers in source frames by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/astral-sh/ruff/pull/4984
* Suggest combining async with statements by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5022
* Improve `TypedDict` conversion logic for shadowed builtins and dunder methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5038
* Support glob patterns in pep8_naming ignore-names by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5024
* Respect all `__all__` definitions for docstring visibility by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5052
* Don't treat annotations as resolved in forward references by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5060
* Consider ignore-names in all pep8 naming rules by [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) in https://github.com/astral-sh/ruff/pull/5079
* Ignore `reimplemented-builtin` if using `await` by [@tjkuson](https://github.com/tjkuson) in https://github.com/astral-sh/ruff/pull/5101
* Allow space in filename for powershell + windows + python module by [@konstin](https://github.com/konstin) in https://github.com/astral-sh/ruff/pull/5115
* Don't treat straight imports of __future__ as `__future__` imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5128
* Remove continuations when deleting statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/astral-sh/ruff/pull/5198
* Fix corner case involving terminal backslash after fixing `W293` by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/astral-sh/ruff/pull/5172
* Fix subprocess.run on Windows Python 3.7 by [@konstin](https://github.com/konstin) in https://github.com/astral-sh/ruff/pull/5220

## New Contributors
* [@rodjunger](https://github.com/rodjunger) made their first contribution in https://github.com/astral-sh/ruff/pull/4929
* [@DavideCanton](https://github.com/DavideCanton) made their first contribution in https://github.com/astral-sh/ruff/pull/4563
* [@Thomasdezeeuw](https://github.com/Thomasdezeeuw) made their first contribution in https://github.com/astral-sh/ruff/pull/5021
* [@adampauls](https://github.com/adampauls) made their first contribution in https://github.com/astral-sh/ruff/pull/4390
* [@tkukushkin](https://github.com/tkukushkin) made their first contribution in https://github.com/astral-sh/ruff/pull/4853
* [@Taybou](https://github.com/Taybou) made their first contribution in https://github.com/astral-sh/ruff/pull/5088
* [@davidszotten](https://github.com/davidszotten) made their first contribution in https://github.com/astral-sh/ruff/pull/5158
* [@dosisod](https://github.com/dosisod) made their first contribution in https://github.com/astral-sh/ruff/pull/5203

**Full Changelog**: https://github.com/astral-sh/ruff/compare/v0.0.272...v0.0.273

[Changes][v0.0.273]


<a name="v0.0.272"></a>
# [v0.0.272](https://github.com/astral-sh/ruff/releases/tag/v0.0.272) - 08 Jun 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Breaking Changes
* Move flake8-fixme rules to FIX prefix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4917

### Rules
* [`flake8-pyi`] Implement PYI050 by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4884

### Bug Fixes
* Avoid attributing runtime references to module-level imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4942
* Skip class scopes when resolving nonlocal references by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4943
* Apply `dict.get` fix before ternary rewrite by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4944
* Handle implicit string concatenations in conversion-flag rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4947
* Make `C413` fix as suggested for `reversed` call by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4891
* ignore if using infinite iterators in `B905` by [@kyoto7250](https://github.com/kyoto7250) in https://github.com/charliermarsh/ruff/pull/4914

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.271...v0.0.272

[Changes][v0.0.272]


<a name="v0.0.271"></a>
# [v0.0.271](https://github.com/astral-sh/ruff/releases/tag/v0.0.271) - 06 Jun 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* Add autofix for flake8-type-checking by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4742
* [`airflow`] Add AIR001: task variable name should be same as task_id arg by [@jlaneve](https://github.com/jlaneve) in https://github.com/charliermarsh/ruff/pull/4687
* [`flake8-bandit`] Implement S609, linux_commands_wildcard_injection by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/4504
* [`flake8-bugbear`] Move duplicate-value rule to flake8-bugbear by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4882
* [`flake8-fixme`] Implement `flake8_fixme` and refactor `TodoDirective` by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4681
* [`flake8-future-annotations`] Implement `FA102` by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/4702
* [`flake8-pyi`] Add PYI024 for `flake8-pyi` plugin by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4756
* [`flake8-pyi`] Add PYI034 for `flake8-pyi` plugin  by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4764
* [`flake8-pyi`] Add `PYI032` rule with autofix by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4695
* [`flake8-pyi`] Add autofix for PYI010 by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4634
* [`flake8-pyi`] Implement PYI029 by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4851
* [`flake8-pyi`] Implement PYI035 by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4820
* [`flake8-pyi`] Implement PYI048 for `flake8-pyi` plugin by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4645
* [`flake8-pyi`] Implement PYI053 by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4770
* [`flake8-pyi`] Implement PYI054 by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4775
* [`flake8-pyi`] Implement `PYI025` by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4791
* [`flake8-pyi`] Implement `PYI045` by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4700
* [`pylint`] Add Pylint rule `C0208` (`use-sequence-for-iteration`) as `PLC0208` (`iteration-over-set`) by [@tjkuson](https://github.com/tjkuson) in https://github.com/charliermarsh/ruff/pull/4706
* [`pylint`] Add autofix for `PLR1701` (repeated-isinstance-calls) by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4792
* [`pylint`] Implement Pylint's `yield-inside-async-function` rule (`PLE1700`)  by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/4668
* [`pylint`] implement E307 for pylint invalid str return type by [@Ryang20718](https://github.com/Ryang20718) in https://github.com/charliermarsh/ruff/pull/4854
* [`ruff`] Lint pyproject.toml by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/4496
* [`tryceratops`] Ignore error calls with `exc_info` in TRY400 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4797

### Settings
* Add `pyflakes.extend-generics` setting by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4677

### Bug Fixes
* Fix PLW3301 false positive single argument nested min/max by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4683
* Handle dotted alias imports to check for implicit imports by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4685
* Flag empty strings in flake8-errmsg rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4745
* Exclude function definition from too-many-statements rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4794
* Preserve quotes in F523 fixer by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4836
* Fix round-tripping of nested functions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4875
* Avoid early-exit in explicit-f-string-type-conversion by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4886
* Avoid no-op fix for nested with expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4906
* Fix UP036 auto-fix error by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4679
* Use class name as range for `B024` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4647
* Change TODO directive detection to work with multiple pound signs on the same line by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4558
* Allow more immutable funcs for RUF009 by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4660
* Avoid using typing-imported symbols for runtime edits by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4649
* Fix `async for` formatting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4675
* Ignore __setattr__ in FBT003 by [@alexfikl](https://github.com/alexfikl) in https://github.com/charliermarsh/ruff/pull/4752
* Include ImportError in non-fixable try-catch imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4793
* Avoid extra newline between diagnostics in grouped mode by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4776
* Avoid enforcing native-literals rule within nested f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4488
* Respect mixed variable assignment in RET504 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4835
* Make FLY002 autofix into a constant string instead of an f-string if all `join()` arguments are strings by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4834
* Add some exceptions for FBT003 ([#3247](https://github.com/astral-sh/ruff/issues/3247)) by [@allisonkarlitskaya](https://github.com/allisonkarlitskaya) in https://github.com/charliermarsh/ruff/pull/4867
* Avoid running RUF100 rules when code contains syntax errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4869
* Avoid index-out-of-bands panic for positional placeholders by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4872
* Remove destructive fixes for F523 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4883
* Respect shadowed exports in `__all__`  by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4885
* Track symbol deletions separately from bindings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4888
* Change fixable_set to include RuleSelector::All/Nursery by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4852

## New Contributors
* [@bersbersbers](https://github.com/bersbersbers) made their first contribution in https://github.com/charliermarsh/ruff/pull/4644
* [@jlaneve](https://github.com/jlaneve) made their first contribution in https://github.com/charliermarsh/ruff/pull/4690
* [@suharnikov](https://github.com/suharnikov) made their first contribution in https://github.com/charliermarsh/ruff/pull/4678
* [@alexfikl](https://github.com/alexfikl) made their first contribution in https://github.com/charliermarsh/ruff/pull/4752
* [@allisonkarlitskaya](https://github.com/allisonkarlitskaya) made their first contribution in https://github.com/charliermarsh/ruff/pull/4867
* [@Ryang20718](https://github.com/Ryang20718) made their first contribution in https://github.com/charliermarsh/ruff/pull/4854
* [@addisoncrump](https://github.com/addisoncrump) made their first contribution in https://github.com/charliermarsh/ruff/pull/4893

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.270...v0.0.271

[Changes][v0.0.271]


<a name="v0.0.270"></a>
# [v0.0.270](https://github.com/astral-sh/ruff/releases/tag/v0.0.270) - 24 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`flake8-bandit`] Implement `paramiko-call` (`S601`) by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/4500
* [`flake8-pyi`] Add autofix for PYI009 by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4583
* [`flake8-pyi`] Implement `PYI013` by [@density](https://github.com/density) in https://github.com/charliermarsh/ruff/pull/4517
* [`pylint`] Add `duplicate-value` (`W0130`) by [@hoel-bagard](https://github.com/hoel-bagard) in https://github.com/charliermarsh/ruff/pull/4515
* [`pylint`] Add `named_expr_without_context` (`W0131`) by [@hoel-bagard](https://github.com/hoel-bagard) in https://github.com/charliermarsh/ruff/pull/4531
* [`ruff`] Extend `RUF005` to recursive and literal-literal concatenations by [@hoel-bagard](https://github.com/hoel-bagard) in https://github.com/charliermarsh/ruff/pull/4557
* [`ruff`] Make ambiguous-unicode detection sensitive to 'word' context by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4552
* [`ruff`] Name ambiguous characters by [@covracer](https://github.com/covracer) in https://github.com/charliermarsh/ruff/pull/4448

### Settings
* Implement `--extend-fixable` option by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4297
* Support new `extend-per-file-ignores` setting by [@aacunningham](https://github.com/aacunningham) in https://github.com/charliermarsh/ruff/pull/4265


### Bug Fixes
* Fix RUF010 auto-fix with parenthesis by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4524
* Parenthesize more sub-expressions in f-string conversion by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4535
* Fix false-positive for TRY302 if exception cause is given by [@153957](https://github.com/153957) in https://github.com/charliermarsh/ruff/pull/4559
* Fix `# isort: split` comment detection in nested blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4584
* Avoid some false positives in dunder variable assigments by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/4508
* Fix UP032 auto-fix with integers by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4525
* Improve reference resolution for deferred-annotations-within-classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4509
* Improve handling of `__qualname__`, `__module__`, and `__class__` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4512
* Include empty success test in JUnit output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4537
* Fix SIM110 and SIM111 ranges by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4545
* Ignore `#region` code folding marks in eradicate rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4546
* Avoid infinite loop for required imports with isort: off by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4581
* Make B007 fix relevance stricter by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4607
* Introduce `tab-size` to correcly calculate the line length with tabulations by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4167
* Visit `TypeVar` and `NewType` name arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4627
* Improve `Message` sorting performance by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4624

## New Contributors
* [@hoel-bagard](https://github.com/hoel-bagard) made their first contribution in https://github.com/charliermarsh/ruff/pull/4516
* [@density](https://github.com/density) made their first contribution in https://github.com/charliermarsh/ruff/pull/4517
* [@Mr-Pepe](https://github.com/Mr-Pepe) made their first contribution in https://github.com/charliermarsh/ruff/pull/4540
* [@153957](https://github.com/153957) made their first contribution in https://github.com/charliermarsh/ruff/pull/4559
* [@covracer](https://github.com/covracer) made their first contribution in https://github.com/charliermarsh/ruff/pull/4448

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.269...v0.0.270

[Changes][v0.0.270]


<a name="v0.0.269"></a>
# [v0.0.269](https://github.com/astral-sh/ruff/releases/tag/v0.0.269) - 18 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

(This is a rerelease of `v0.0.268`, which didn't make it to PyPI due to user error. As such, the release notes are copied below.)

### `pycodestyle`

This release includes optimized implementations of a large portion of `pycodestyle`, for those that use Ruff without an autoformatter.

In this initial release, the rules are being introduced under a "nursery" flag, which requires that users explicitly select them (e.g., `select = ["E111"]`); in other words, these rules are not yet enabled via `select = ["E"]`.

If you're interested in testing the `pycodestyle` rules, you can enable them via:

```toml
select = [
    "E111", "E112", "E113", "E114", "E115", "E116", "E117",
    "E201", "E202", "E203", "E211", "E221", "E222", "E223",
    "E224", "E225", "E226", "E227", "E228", "E231", "E251",
    "E252", "E261", "E262", "E265", "E266", "E271", "E272",
    "E273", "E274", "E275",
]
```

These rules will be included as part of the `E` category in a future release.

### Breaking Changes
* [`pyupgrade`] Remove `keep-runtime-typing` setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4427

### Rules
* [`pylint`] Add `duplicate-bases` rule by [@alonme](https://github.com/alonme) in https://github.com/charliermarsh/ruff/pull/4411
* [`pylint`] Fix `PLW3301` auto-fix with generators by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4412
* [`flake8-async`] Implement flake8-async plugin by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4432
* [`pyupgrade`] Enable automatic rewrites of `typing.Deque` and `typing.DefaultDict` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4420
* [`flake8-pyi`] Implement `unannotated-assignment-in-stub` (`PY052`) by [@sladyn98](https://github.com/sladyn98) in https://github.com/charliermarsh/ruff/pull/4293
* [`tryceratops`] Implement TRY302 - `raise` after `except` by [@john-h-k](https://github.com/john-h-k) in https://github.com/charliermarsh/ruff/pull/4461
* [`flake8-bandit`] Improve SQL injection detection logic (`S608`) by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/4499
* [`flake8-todos`] Implement `flake8_todos` by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/3921
* [`flake8-future-annotations`] Implement `flake8-future-annotations` FA100 by [@TylerYep](https://github.com/TylerYep) in https://github.com/charliermarsh/ruff/pull/3979
* Enable `pycodestyle` rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3689
* Enable `pycodestyle` rules under new "nursery" category by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4407

### Settings
* Merge subsettings when extending configurations by [@bendoerry](https://github.com/bendoerry) in https://github.com/charliermarsh/ruff/pull/4431

### Bug Fixes
* Extend multi-line noqa directives to start-of-line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4490
* Fix scoping of comprehensions within classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4494
* Enable autofix for split-assertions at top level by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4405
* Ignore ANN401 for overridden methods by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4409
* Fix `RUF010` autofix within f-strings by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4423
* Update C419 to be a suggested fix by [@madkinsz](https://github.com/madkinsz) in https://github.com/charliermarsh/ruff/pull/4424
* Fix expected-indentation errors with end-of-line comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4438
* Emit non-logical newlines for "empty" lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4444
* Avoid emitting empty logical lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4452
* Avoid flagging missing whitespace for decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4454
* Remove special-casing for whitespace-around-@ by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4458
* Avoid triggering `pd#at` and friends on non-subscripts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4474
* Include precise tokens for extraneous-whitespace diagnostics by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4471
* Allow shebang comments at start-of-file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4473
* Bring pycodestyle rules into full compatibility (on SciPy) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4472
* Invert quote-style when generating code within f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4487
* Fix COM812 false positive in string subscript by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4493
* Overhaul sdist handling by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/4439

## New Contributors
* [@TylerYep](https://github.com/TylerYep) made their first contribution in https://github.com/charliermarsh/ruff/pull/3979
* [@yanksyoon](https://github.com/yanksyoon) made their first contribution in https://github.com/charliermarsh/ruff/pull/4428
* [@bendoerry](https://github.com/bendoerry) made their first contribution in https://github.com/charliermarsh/ruff/pull/4431
* [@qdegraaf](https://github.com/qdegraaf) made their first contribution in https://github.com/charliermarsh/ruff/pull/4432
* [@jameslamb](https://github.com/jameslamb) made their first contribution in https://github.com/charliermarsh/ruff/pull/4446
* [@john-h-k](https://github.com/john-h-k) made their first contribution in https://github.com/charliermarsh/ruff/pull/4461

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.267...v0.0.269

[Changes][v0.0.269]


<a name="v0.0.268"></a>
# [v0.0.268](https://github.com/astral-sh/ruff/releases/tag/v0.0.268) - 18 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### `pycodestyle`

This release includes optimized implementations of a large portion of `pycodestyle`, for those that use Ruff without an autoformatter.

In this initial release, the rules are being introduced under a "nursery" flag, which requires that users explicitly select them (e.g., `select = ["E111"]`); in other words, these rules are not yet enabled via `select = ["E"]`.

If you're interested in testing the `pycodestyle` rules, you can enable them via:

```toml
select = [
    "E111", "E112", "E113", "E114", "E115", "E116", "E117",
    "E201", "E202", "E203", "E211", "E221", "E222", "E223",
    "E224", "E225", "E226", "E227", "E228", "E231", "E251",
    "E252", "E261", "E262", "E265", "E266", "E271", "E272",
    "E273", "E274", "E275",
]
```

These rules will be included as part of the `E` category in a future release.

### Breaking Changes
* [`pyupgrade`] Remove `keep-runtime-typing` setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4427

### Rules
* [`pylint`] Add `duplicate-bases` rule by [@alonme](https://github.com/alonme) in https://github.com/charliermarsh/ruff/pull/4411
* [`pylint`] Fix `PLW3301` auto-fix with generators by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4412
* [`flake8-async`] Implement flake8-async plugin by [@qdegraaf](https://github.com/qdegraaf) in https://github.com/charliermarsh/ruff/pull/4432
* [`pyupgrade`] Enable automatic rewrites of `typing.Deque` and `typing.DefaultDict` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4420
* [`flake8-pyi`] Implement `unannotated-assignment-in-stub` (`PY052`) by [@sladyn98](https://github.com/sladyn98) in https://github.com/charliermarsh/ruff/pull/4293
* [`tryceratops`] Implement TRY302 - `raise` after `except` by [@john-h-k](https://github.com/john-h-k) in https://github.com/charliermarsh/ruff/pull/4461
* [`flake8-bandit`] Improve SQL injection detection logic (`S608`) by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/4499
* [`flake8-todos`] Implement `flake8_todos` by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/3921
* [`flake8-future-annotations`] Implement `flake8-future-annotations` FA100 by [@TylerYep](https://github.com/TylerYep) in https://github.com/charliermarsh/ruff/pull/3979
* Enable `pycodestyle` rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3689
* Enable `pycodestyle` rules under new "nursery" category by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4407

### Settings
* Merge subsettings when extending configurations by [@bendoerry](https://github.com/bendoerry) in https://github.com/charliermarsh/ruff/pull/4431

### Bug Fixes
* Extend multi-line noqa directives to start-of-line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4490
* Fix scoping of comprehensions within classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4494
* Enable autofix for split-assertions at top level by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4405
* Ignore ANN401 for overridden methods by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4409
* Fix `RUF010` autofix within f-strings by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4423
* Update C419 to be a suggested fix by [@madkinsz](https://github.com/madkinsz) in https://github.com/charliermarsh/ruff/pull/4424
* Fix expected-indentation errors with end-of-line comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4438
* Emit non-logical newlines for "empty" lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4444
* Avoid emitting empty logical lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4452
* Avoid flagging missing whitespace for decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4454
* Remove special-casing for whitespace-around-@ by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4458
* Avoid triggering `pd#at` and friends on non-subscripts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4474
* Include precise tokens for extraneous-whitespace diagnostics by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4471
* Allow shebang comments at start-of-file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4473
* Bring pycodestyle rules into full compatibility (on SciPy) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4472
* Invert quote-style when generating code within f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4487
* Fix COM812 false positive in string subscript by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4493
* Overhaul sdist handling by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/4439

## New Contributors
* [@TylerYep](https://github.com/TylerYep) made their first contribution in https://github.com/charliermarsh/ruff/pull/3979
* [@yanksyoon](https://github.com/yanksyoon) made their first contribution in https://github.com/charliermarsh/ruff/pull/4428
* [@bendoerry](https://github.com/bendoerry) made their first contribution in https://github.com/charliermarsh/ruff/pull/4431
* [@qdegraaf](https://github.com/qdegraaf) made their first contribution in https://github.com/charliermarsh/ruff/pull/4432
* [@jameslamb](https://github.com/jameslamb) made their first contribution in https://github.com/charliermarsh/ruff/pull/4446
* [@john-h-k](https://github.com/john-h-k) made their first contribution in https://github.com/charliermarsh/ruff/pull/4461

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.267...v0.0.268

[Changes][v0.0.268]


<a name="v0.0.267"></a>
# [v0.0.267](https://github.com/astral-sh/ruff/releases/tag/v0.0.267) - 12 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## Summary

Follow-up release to v0.0.266 to fix an issue with `python -m ruff`- and `import ruff`-based workflows.

(No new rules or functionality.)

## What's Changed

### Rules
* Implement `RUF010` to detect explicit type conversions within f-strings by [@LotemAm](https://github.com/LotemAm) in https://github.com/charliermarsh/ruff/pull/4387

### Other Changes
* Workaround for maturin bug by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/4399

## New Contributors
* [@OMEGARAZER](https://github.com/OMEGARAZER) made their first contribution in https://github.com/charliermarsh/ruff/pull/3938
* [@LotemAm](https://github.com/LotemAm) made their first contribution in https://github.com/charliermarsh/ruff/pull/4387

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.266...v0.0.267

[Changes][v0.0.267]


<a name="v0.0.266"></a>
# [v0.0.266](https://github.com/astral-sh/ruff/releases/tag/v0.0.266) - 12 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Breaking Changes
* Remove deprecated `update-check` setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4313
* JSON Emitter: Use one indexed column numbers for edits by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4007

### Rules
* [`pygrep-hooks`] Implement pygrep-hook's Mock-mistake diagnostic by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4366
* [`pylint`] Implement `nested-min-max` (`W3301`) by [@mccullocht](https://github.com/mccullocht) in https://github.com/charliermarsh/ruff/pull/4200
* [`flynt`] Implement Flynt static string join transform as FLY002 by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/4196
* [`pylint`] Include positional- and keyword-only arguments in too-many-arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4329
* [`ruff`] Update confusable character mapping by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/4274

### Settings
* Add .git-rewrite folder to default ignored folder paths by [@jleclanche](https://github.com/jleclanche) in https://github.com/charliermarsh/ruff/pull/4261
* Feat: detect changes also in configuration files by [@mikeleppane](https://github.com/mikeleppane) in https://github.com/charliermarsh/ruff/pull/4169

### Bug Fixes
* Revert the B027 autofix logic by [@aacunningham](https://github.com/aacunningham) in https://github.com/charliermarsh/ruff/pull/4310
* Consider Flask app logger as logger candidate by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4253
* Enforce max-doc-length for multi-line docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4347
* Avoid re-using imports beyond current edit site by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4378
* Respect insertion location when importing symbols by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4258
* Fix jemalloc page size on aarch64 by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4247
* Fix replace_whitespace() tabulation to space by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4226
* Avoid fixing `PD002` in a lambda expression by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4286
* Avoid `D403` if first char cannot be uppercased by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4283
* Avoid panics for f-string rewrites at start-of-file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4291
* Rewrite `not not a` as `bool(a)` in boolean contexts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4294
* Include static and class methods in in abstract decorator list by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4298
* Specify exact command in incorrect parentheses suggestion by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4300
* Ignore `TRY301` exceptions without except handlers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4301
* Preserve whitespace around `ListComp` brackets in `C419` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4099
* Tweak capitalization of B021 message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4350
* Avoid debug panic with empty indent replacement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4364
* Use target name in hardcoded-password diagnostics by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4365
* Avoid underflow in expected-special-method-signature by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4377
* Respect `__all__` imports when determining definition visibility by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4357
* Ignore some methods on list in `flake8-boolean-trap` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4385
* Fix false positives in PD002 by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4337
* Run autofix on initial watcher pass by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4311
* Avoid SIM105 autofixes that would remove comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4330
* Handle `.encode` calls on parenthesized expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4338
* Truncate `SyntaxError`s before newline character by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4124
* Use non-empty ranges for logical-lines diagnostics by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4133

## New Contributors
* [@jleclanche](https://github.com/jleclanche) made their first contribution in https://github.com/charliermarsh/ruff/pull/4261
* [@aureliojargas](https://github.com/aureliojargas) made their first contribution in https://github.com/charliermarsh/ruff/pull/4306
* [@intgr](https://github.com/intgr) made their first contribution in https://github.com/charliermarsh/ruff/pull/4304
* [@mikeleppane](https://github.com/mikeleppane) made their first contribution in https://github.com/charliermarsh/ruff/pull/4169
* [@dependabot](https://github.com/dependabot) made their first contribution in https://github.com/charliermarsh/ruff/pull/4354

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.265...v0.0.266

[Changes][v0.0.266]


<a name="v0.0.265"></a>
# [v0.0.265](https://github.com/astral-sh/ruff/releases/tag/v0.0.265) - 05 May 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Breaking Changes
* Change `--fix-only` exit semantics to mirror `--fix` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4146

### Rules
* [flake8-pyi] PYI020 by [@arya-k](https://github.com/arya-k) in https://github.com/charliermarsh/ruff/pull/4211
* Update B027 to support autofixing by [@aacunningham](https://github.com/aacunningham) in https://github.com/charliermarsh/ruff/pull/4178
* [`flake8-pyi`] Implement `PYI042` and `PYI043` by [@arya-k](https://github.com/arya-k) in https://github.com/charliermarsh/ruff/pull/4214
* [`pylint`] Implement import-self (`W0406`) by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/4154
* Warn on PEP 604 syntax not in an annotation, but don't autofix by [@wookie184](https://github.com/wookie184) in https://github.com/charliermarsh/ruff/pull/4170

### Bug Fixes
* Fix panic in pydocstyle D214 when docstring indentation is empty by [@madkinsz](https://github.com/madkinsz) in https://github.com/charliermarsh/ruff/pull/4216
* Render tabs as 4 spaces in diagnostics by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4132
* Fix era panic caused by out of bound edition by [@leiserfg](https://github.com/leiserfg) in https://github.com/charliermarsh/ruff/pull/4206
* End of statement insertion should occur after newline by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4215
* Ignore __debuggerskip__ in unused variable checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4229

### CLI
* Show settings path in `--show-settings` output by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4199

### Documentation
* Allow linking to individual rules by [@calumy](https://github.com/calumy) in https://github.com/charliermarsh/ruff/pull/4158

## New Contributors
* [@wookie184](https://github.com/wookie184) made their first contribution in https://github.com/charliermarsh/ruff/pull/4170
* [@arya-k](https://github.com/arya-k) made their first contribution in https://github.com/charliermarsh/ruff/pull/4211

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.264...v0.0.265

[Changes][v0.0.265]


<a name="v0.0.264"></a>
# [v0.0.264](https://github.com/astral-sh/ruff/releases/tag/v0.0.264) - 02 May 2023

<!-- Release notes generated using configuration in .github/release.yml at 8cb76f85eba1c970a8c800348fd1e0c874621a57 -->

## What's Changed

### Rules
* Autofix `EM101`, `EM102`, `EM103` if possible by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4123
* Add bugbear immutable functions as allowed in dataclasses by [@mosauter](https://github.com/mosauter) in https://github.com/charliermarsh/ruff/pull/4122

### Settings
* Add support for providing command-line arguments via `argfile` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4087

### Bug Fixes
* Make D410/D411 autofixes mutually exclusive by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4110
* Remove `pyright` comment prefix from PYI033 checks by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4152
* Fix F811 false positive with match by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4161
* Fix `E713` and `E714` false positives for multiple comparisons by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4083
* Fix B023 shadowed variables in nested functions by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4111
* Preserve star-handling special-casing for force-single-line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4129
* Respect parent-scoping rules for `NamedExpr` assignments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4145
* Fix UP032 auto-fix by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4165
* Allow boolean parameters for `pytest.param` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4176

### Internal
* Replace row/column based `Location` with byte-offsets. by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3931
* perf(logical-lines): Various small perf improvements by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4022
* Use `memchr` to speedup newline search on x86 by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3985
* Remove `ScopeStack` in favor of child-parent `ScopeId` pointers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4138

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.263...v0.0.264

[Changes][v0.0.264]


<a name="v0.0.263"></a>
# [v0.0.263](https://github.com/astral-sh/ruff/releases/tag/v0.0.263) - 25 Apr 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`flake8-bugbear`] Add `pytest.raises(Exception)` support to B017 by [@alanhdu](https://github.com/alanhdu) in https://github.com/charliermarsh/ruff/pull/4052
* [`flake8-import-conventions`] Implement new rule `ICN003` to ban `from ... import ...` for selected modules by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/4040
* [`pylint`] Implement PLE0302 `unexpected-special-method-signature` by [@mccullocht](https://github.com/mccullocht) in https://github.com/charliermarsh/ruff/pull/4075
* [`pep8-naming`] Ignore `N815` for `TypedDict` fields by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4066

### Bug Fixes
* Avoid `PYI015` for valid default value without annotation by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4043
* Avoid infinite-propagation of inline comments when force-splitting imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4074
* Fix SIM222 and SIM223 false positives and auto-fix by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/4063
* Unify positional and keyword arguments when checking for missing arguments in docstring by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4067
* Avoid `RUF008` if field annotation is immutable by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4039
* Increment priority should be (branch-local, global) by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4070
* Ignore `ClassVar` annotation for `RUF008`, `RUF009` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/4081
* Support --fix in watch mode by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/4035

## New Contributors
* [@alanhdu](https://github.com/alanhdu) made their first contribution in https://github.com/charliermarsh/ruff/pull/4052
* [@pronoym99](https://github.com/pronoym99) made their first contribution in https://github.com/charliermarsh/ruff/pull/4055
* [@Secrus](https://github.com/Secrus) made their first contribution in https://github.com/charliermarsh/ruff/pull/4085
* [@madkinsz](https://github.com/madkinsz) made their first contribution in https://github.com/charliermarsh/ruff/pull/4084
* [@mccullocht](https://github.com/mccullocht) made their first contribution in https://github.com/charliermarsh/ruff/pull/4075

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.262...v0.0.263

[Changes][v0.0.263]


<a name="v0.0.262"></a>
# [v0.0.262](https://github.com/astral-sh/ruff/releases/tag/v0.0.262) - 19 Apr 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Configuration
* Allow users to extend the set of included files via `include` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3914
* Implement isort custom sections and ordering ([#2419](https://github.com/astral-sh/ruff/issues/2419)) by [@hackedd](https://github.com/hackedd) in https://github.com/charliermarsh/ruff/pull/3900

### Rules
* [`flake8-simplify`] Add autofix for `contextlib.suppress` (`SIM105`) by [@leiserfg](https://github.com/leiserfg) in https://github.com/charliermarsh/ruff/pull/3915
* [`flake8-bandit`] Ignore assert errors (S101) in `TYPE_CHECKING` blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3960
* [`flake8-comprehensions`] Implement `unnecessary-literal-within-dict-call` (`C418`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3969
* [`ruff`] Add checks for mutable defaults `dataclass`es by [@mosauter](https://github.com/mosauter) in https://github.com/charliermarsh/ruff/pull/3877
* [`flake8-import-conventions`] Add a rule for `BannedImportAlias` by [@stancld](https://github.com/stancld) in https://github.com/charliermarsh/ruff/pull/3926
* [`flake8-pyi`] Implement duplicate types in unions (`PYI016`) by [@USER-5](https://github.com/USER-5) in https://github.com/charliermarsh/ruff/pull/3922
* [`flake8-bandit`] Implement flake8-bandit shell injection rules by [@robyoung](https://github.com/robyoung) in https://github.com/charliermarsh/ruff/pull/3924
* [`flake8-comprehensions`] Redirect `PIE802` to `C419` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3971

### Bug Fixes
* Fix unicode handling in PLE2515 by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/3898
* Avoid adding required imports to stub files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3940
* Add 'or if cond' to `E712` message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3962
* Ignore argument assignments when enforcing `RET504` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4004
* Fix (doc-)line-too-long start location by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4006
* Ignore stub file assignments to value-requiring targets by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4030
* Allow legacy C and T selectors in JSON schema by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3889
* Ignore `PLW2901` when using typing cast by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3891
* Visit comprehension to detect group name usage/overrides by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3887
* Ensure that tab characters aren't in multi-line strings before throwing a violation by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/3837
* Avoid N802 violations for `@override` methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3912
* Check for arguments in inner/outer call for `C414` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3916
* Do not skip analysis if `*args` present for `F523` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3923
* Extend SIM105 to match also 'Ellipsis only' bodies in exception handlers by [@leiserfg](https://github.com/leiserfg) in https://github.com/charliermarsh/ruff/pull/3925
* Support `pyright: ignore` comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3941
* Tidy up some `pygrep-hooks` rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3942
* Use identifier range for pytest rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3948
* Allow `typing_extensions.TypeVar` assignments in `.pyi` files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3951
* Raise percent-format upgrade rule (`UP031`) for hanging modulos by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3953
* Check for parenthesis in implicit str concat in `PT006` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3955
* Do not consider nested comment as part of code by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3984
* Preserve type annotations when fixing `E731` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3983
* Remove autofix behavior for uncapitalized-environment-variables (`SIM112`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3988
* Respect typing-modules when evaluating no-return functions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4001
* Avoid short-circuiting when detecting RET rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4002
* Set non-empty range for indentation diagnostics by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/4005
* Ignore relative imports in `banned-api` rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4024
* Support relative imports in `banned-api` enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4025
* Treat non-future function annotations as required-at-runtime by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4028
* Ignore certain flake8-pyi errors within function bodies by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/4029

## New Contributors
* [@tjkuson](https://github.com/tjkuson) made their first contribution in https://github.com/charliermarsh/ruff/pull/3886
* [@mosauter](https://github.com/mosauter) made their first contribution in https://github.com/charliermarsh/ruff/pull/3877
* [@stancld](https://github.com/stancld) made their first contribution in https://github.com/charliermarsh/ruff/pull/3926
* [@USER-5](https://github.com/USER-5) made their first contribution in https://github.com/charliermarsh/ruff/pull/3922
* [@robyoung](https://github.com/robyoung) made their first contribution in https://github.com/charliermarsh/ruff/pull/3924
* [@hackedd](https://github.com/hackedd) made their first contribution in https://github.com/charliermarsh/ruff/pull/3900
* [@justinchuby](https://github.com/justinchuby) made their first contribution in https://github.com/charliermarsh/ruff/pull/3982
* [@mirecl](https://github.com/mirecl) made their first contribution in https://github.com/charliermarsh/ruff/pull/4008
* [@Xemnas0](https://github.com/Xemnas0) made their first contribution in https://github.com/charliermarsh/ruff/pull/4026

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.261...v0.0.262

[Changes][v0.0.262]


<a name="v0.0.261"></a>
# [v0.0.261](https://github.com/astral-sh/ruff/releases/tag/v0.0.261) - 05 Apr 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`flake8-simplify`] Ignore `collapsible-if` violations for `if False:` and `if True:` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3732
* [`flake8-pie`] Extend `unncessary-generator-any-all` to set comprehensions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3824
* [`flake8-simplify`] Implement `dict-get-with-none-default` (`SIM910`) by [@kyoto7250](https://github.com/kyoto7250) in https://github.com/charliermarsh/ruff/pull/3874
* [`flake8-annotations`] Additional simple magic return types by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3805
* [`flake8-pyi`]: fix PYI015 false positive on assignment of TypeVar & friends by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/3861

### Bug Fixes
* Improve robustness of argument removal for `encode` calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3802
* Fix SIM222 and SIM223 false positive by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3832
* When checking module visibility, don't check entire ancestry by [@Hnasar](https://github.com/Hnasar) in https://github.com/charliermarsh/ruff/pull/3835
* Improve top-of-file insertions for required imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3779
* Use multi-fix semantics for `inplace` removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3804
* Flag non-`Name` expressions in `duplicate-isinstance-call` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3817
* Avoid `unnecessary-comprehension-any-all` for async generators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3823
* Fix `is_module_name()` and improve perf of `is_identifier()` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3795
* Allow starred arguments in B030 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3871
* Support mutually exclusive branches for `B031` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3844
* Consider logger candidate from `logging` module only by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3878

### Core
* Add import insertion support to autofix capabilities by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3787
* Generate `ImportMap` from module path to imported dependencies by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/3243

### Docs
* Add documentation for `ruff-action` (GitHub Action!) by [@brucearctor](https://github.com/brucearctor) in https://github.com/charliermarsh/ruff/pull/3857

## New Contributors
* [@AetherUnbound](https://github.com/AetherUnbound) made their first contribution in https://github.com/charliermarsh/ruff/pull/3806
* [@Hnasar](https://github.com/Hnasar) made their first contribution in https://github.com/charliermarsh/ruff/pull/3835
* [@nvuillam](https://github.com/nvuillam) made their first contribution in https://github.com/charliermarsh/ruff/pull/3848
* [@brucearctor](https://github.com/brucearctor) made their first contribution in https://github.com/charliermarsh/ruff/pull/3857

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.260...v0.0.261

[Changes][v0.0.261]


<a name="v0.0.260"></a>
# [v0.0.260](https://github.com/astral-sh/ruff/releases/tag/v0.0.260) - 29 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`flake8-bugbear`] Add more immutable functions for `B008` by [@rouge8](https://github.com/rouge8) in https://github.com/charliermarsh/ruff/pull/3764
* [`flake8-bugbear`] Allow `pathlib.Path()` in `B008` by [@rouge8](https://github.com/rouge8) in https://github.com/charliermarsh/ruff/pull/3794
* [`flake8-bugbear`] Expand the scope of useless-expression (B018) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3455
* [`flake8-bugbear`]: Implement rule `B031` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3680
* [`flake8-gettext`] Implement `flake8-gettext` by [@leiserfg](https://github.com/leiserfg) in https://github.com/charliermarsh/ruff/pull/3785
* [`flake8-logging-format`] Add support for `.log(level, msg)` calls in `flake8-logging-format` by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3726
* [`flake8-logging-format`] Allow aliased `logging` module as a logger candidate by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3718
* [`flake8-pyi`] Add autofix for `PYI014` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3729
* [`flake8-pyi`] Implement `PYI012` by [@JBLDKY](https://github.com/JBLDKY) in https://github.com/charliermarsh/ruff/pull/3743
* [`flake8-pyi`] Implement `PYI015` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3728
* [`flake8-simplify`] Fix SIM222 and SIM223 false negative by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3740
* [`isort`]: support submodules in known_(first|third)_party config options by [@astaric](https://github.com/astaric) in https://github.com/charliermarsh/ruff/pull/3768
* [`pycodestyle`] Use unicode-width to determine line-length instead of character count by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3714
* [`pydocstyle`] Implement autofix for `D403` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3731
* [`pylint`] Avoid `useless-import alias` (`C0414`) in `.pyi` files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3761
* [`pylint`] Exempt `PLR1711` and `RET501` if non-`None` annotation by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3705
* [`tryceratops`] Exempt return with side effects for TRY300 by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3780

### Bug Fixes
* Avoid parsing `ForwardRef` contents as type references by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3698
* Avoid parsing f-strings in type annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3699
* Include `with` statements in complexity calculation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3771
* Use import alias locations for `pep8-naming` import rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3772
* Allow `TID252` to fix all valid module paths by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3796
* Fix SIM118 auto-fix by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3695
* Avoid panics for implicitly concatenated forward references by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3700
* Allow simple container literals as default values by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3703
* Traverse over nested string type annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3724
* Use `wild::args()` and add `wild` as a dependency by [@agriyakhetarpal](https://github.com/agriyakhetarpal) in https://github.com/charliermarsh/ruff/pull/3739
* Avoid overlong-line errors for lines that end with URLs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3663
* Sort statistics by count by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3748
* Reduce explicit clones by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3793
* Add flymake-ruff to docs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3800

## New Contributors
* [@agriyakhetarpal](https://github.com/agriyakhetarpal) made their first contribution in https://github.com/charliermarsh/ruff/pull/3739
* [@leiserfg](https://github.com/leiserfg) made their first contribution in https://github.com/charliermarsh/ruff/pull/3741
* [@JBLDKY](https://github.com/JBLDKY) made their first contribution in https://github.com/charliermarsh/ruff/pull/3743
* [@astaric](https://github.com/astaric) made their first contribution in https://github.com/charliermarsh/ruff/pull/3768

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.259...v0.0.260

[Changes][v0.0.260]


<a name="v0.0.259"></a>
# [v0.0.259](https://github.com/astral-sh/ruff/releases/tag/v0.0.259) - 23 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## Summary

Follow-up release to `v0.0.258` to fix an issue related to rule resolution via `select` and `ignore`.

## What's Changed

### Bug Fixes
* Fix RuleSet.remove by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3685
* Respect all rule-exemption sources when suppressing parser errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3665
* Avoid nested loops in `missing_whitespace` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3688

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.258...v0.0.259

[Changes][v0.0.259]


<a name="v0.0.258"></a>
# [v0.0.258](https://github.com/astral-sh/ruff/releases/tag/v0.0.258) - 22 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`flake8-comprehensions`] Update `C416` with dict comprehension (autofixable) by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3605
* [`pylint`]: Implement `assert-on-string-literal` (`W0129`) by [@latonis](https://github.com/latonis) in https://github.com/charliermarsh/ruff/pull/3610
* [`pyupgrade`] Convert single-argument %-style format calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3600
* [`pyupgrade`] Flag PEP 585 and PEP 604 violations in quoted annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3593
* [`pyupgrade`] Enable autofix for annotations within 'simple' string literals by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3657
* [`pyflakes`] Add autofix functionality for `F523` ([#3613](https://github.com/astral-sh/ruff/issues/3613)) by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3613
* [`flake8-bandit`]: Implement deny-list rules for suspicious member calls by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/3239
* [`flake8-annotations`] Add autofix for `ANN204` with magic methods by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3633
* [`pylint`] Implement `binary-op-exception` (`PLW0711`) by [@latonis](https://github.com/latonis) in https://github.com/charliermarsh/ruff/pull/3639
* [`flake8-django`]: Implement rule DJ012 by [@dhruvmanila](https://github.com/dhruvmanila) in https://github.com/charliermarsh/ruff/pull/3659

### Bug Fixes
* Check exclusions prior to resolving `pyproject.toml` files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3588
* Fix D417 false positive by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3596
* Avoid removing comment hash for noqa's with trailing content by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3589
* Avoid panics for implicitly-concatenated docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3584
* Fix infinite loop due to rules `D207` & `W605` by [@vlindhol](https://github.com/vlindhol) in https://github.com/charliermarsh/ruff/pull/3609
* Avoid trimming escaped whitespace in D210 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3635
* Handle `UP032` autofix with adjacent keywords by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3636
* Consider same-site fixes to be overlapping by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3638
* Avoid `RUF007` fixes for more than two arguments by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3654
* Allow `pairwise` diagnostics for `zip(..., strict=True)` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3669
* isort: fix bad interaction between `force-sort-within-sections` and `force-to-top` by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/3645
* Gracefully handle lint panics by [@MichaReiser](https://github.com/MichaReiser) in https://github.com/charliermarsh/ruff/pull/3509
* Fix TRY300 false positive by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3634
* Avoid raising PEP 604 errors with forward-referenced members by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3640
* Avoid attempting infinite `open` fix with re-bound builtin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3650
* Check indentation level when executing E231 by [@kyoto7250](https://github.com/kyoto7250) in https://github.com/charliermarsh/ruff/pull/3668
* Flag, but don't fix, unused imports (`F401`) in `ModuleNotFoundError` blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3658

## New Contributors
* [@Rogdham](https://github.com/Rogdham) made their first contribution in https://github.com/charliermarsh/ruff/pull/3607
* [@vlindhol](https://github.com/vlindhol) made their first contribution in https://github.com/charliermarsh/ruff/pull/3609
* [@dhruvmanila](https://github.com/dhruvmanila) made their first contribution in https://github.com/charliermarsh/ruff/pull/3605
* [@luke396](https://github.com/luke396) made their first contribution in https://github.com/charliermarsh/ruff/pull/3604
* [@fuziontech](https://github.com/fuziontech) made their first contribution in https://github.com/charliermarsh/ruff/pull/3641

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.257...v0.0.258

[Changes][v0.0.258]


<a name="v0.0.257"></a>
# [v0.0.257](https://github.com/astral-sh/ruff/releases/tag/v0.0.257) - 18 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`ruff`] Prefer `itertools.pairwise()` over `zip()` for successive pairs (`RUF007`) by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/3501
* [`flake8-bugbear`] Add `no-explicit-stacklevel` (`B028`) by [@johnor](https://github.com/johnor) in https://github.com/charliermarsh/ruff/pull/3550
* [`pylint`] invalid-characters-* by [@r3m0t](https://github.com/r3m0t) in https://github.com/charliermarsh/ruff/pull/3552
* [`pylint`] Implement `useless-return` (`R1711`) by [@tomecki](https://github.com/tomecki) in https://github.com/charliermarsh/ruff/pull/3116
* [`pylint`]: Implement `continue-in-finally` (`E0116`) by [@latonis](https://github.com/latonis) in https://github.com/charliermarsh/ruff/pull/3541

### Bug Fixes
* Rewrite mock import with starred imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3566
* Respect `type` overrides in E721 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3582
* Use `value > max` style in pylint and mccabe messages by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/3553
* Fix autofix conflict between `D209` and `D400` by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3564
* Avoid C1901 violations within subscripts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3517
* Avoid adding dashed line outside of docstring by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3581
* Enable ANSI colors on Windows 10 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3583

## New Contributors
* [@johnor](https://github.com/johnor) made their first contribution in https://github.com/charliermarsh/ruff/pull/3550

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.256...v0.0.257

[Changes][v0.0.257]


<a name="v0.0.256"></a>
# [v0.0.256](https://github.com/astral-sh/ruff/releases/tag/v0.0.256) - 15 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Bug Fixes
* PYI011: allow `math` constants in defaults by [@XuehaiPan](https://github.com/XuehaiPan) in https://github.com/charliermarsh/ruff/pull/3484
* Remove erroneous C4-to-C40 redirect by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3488
* fix lack of `not`  in error message by [@Czaki](https://github.com/Czaki) in https://github.com/charliermarsh/ruff/pull/3497
* Ensure that redirect warnings appear exactly once per code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3500
* Allow f-strings and concatenations in os.getenv by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3516
* Allow string percent formatting in os.getenv by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3518
* Refine complexity rules for try-except-else-finally by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3519
* Replicate inline comments when splitting single-line imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3521
* Avoid PEP 604 isinstance errors for starred tuples by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3527
* Avoid tracking as-imports separately with force-single-line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3530
* Fix PYI011 and add auto-fix by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3492
* Avoid PEP 604 panic with empty tuple by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3526
* Add last remaining deprecated typing imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3529
* Avoid unused argument violations in .pyi files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3533

### Other Changes
* Include individual path checks in --verbose logging by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3489
* Allow `# ruff:` prefix for isort action comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3493

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.255...v0.0.256

[Changes][v0.0.256]


<a name="v0.0.255"></a>
# [v0.0.255](https://github.com/astral-sh/ruff/releases/tag/v0.0.255) - 13 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Rules
* [`flake8-pie`] Fix PIE802 broken auto-fix with trailing comma by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3402
* [`flake8-pie`] Implement autofix for PIE810 by [@kyoto7250](https://github.com/kyoto7250) in https://github.com/charliermarsh/ruff/pull/3411
* [`flake8-bugbear`] Add `flake8-bugbear`'s B030 rule by [@aacunningham](https://github.com/aacunningham) in https://github.com/charliermarsh/ruff/pull/3400
* [`pycodestyle`] Add E231 by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3344
* [`pyupgrade`] Flag deprecated (but renamed) imports in UP035 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3448
* [`pyupgrade`] Remap ChainMap, Counter, and OrderedDict imports to collections by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3392
* [`pylint`] C1901: compare-to-empty-string by [@AreamanM](https://github.com/AreamanM) in https://github.com/charliermarsh/ruff/pull/3405
* [`pylint`] Implement W1508 invalid-envvar-default by [@latonis](https://github.com/latonis) in https://github.com/charliermarsh/ruff/pull/3449
* [`pylint`] Implement E1507 invalid-envvar-value by [@latonis](https://github.com/latonis) in https://github.com/charliermarsh/ruff/pull/3467

### Settings
* Infer `target-version` from project metadata by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3470
* Implement configuration options `runtime-evaluated-decorators` and `runtime-evaluated-base-classes` for `flake8-type-checking` by [@sasanjac](https://github.com/sasanjac) in https://github.com/charliermarsh/ruff/pull/3292
* Add Azure DevOps as a `--format` option. by [@StefanBRas](https://github.com/StefanBRas) in https://github.com/charliermarsh/ruff/pull/3335

### Bug Fixes
* Re-enable the T and C linter prefix selectors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3452
* Treat unary operations on constants as constant-like by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3348
* Skip byte-order-mark at start of file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3343
* Don't enforce typing-import rules in .pyi files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3362
* Include entire prefix when reporting rule selector errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3375
* Handle multi-line fixes for byte-string prefixing by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3391
* Catch RET504 usages via decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3395
* Ignore multiply-assigned variables in RET504 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3393
* [FIX] PYI011: recognize `Bool` / `Float` / `Complex` numbers as simple defaults by [@XuehaiPan](https://github.com/XuehaiPan) in https://github.com/charliermarsh/ruff/pull/3459
* Respect ignores for runtime-import-in-type-checking-block (TCH004) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3474
* Avoid panicking in invalid_escape_sequence by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3338
* fix: Emit a more useful error if an `extend` points at a non-existent ruff.toml file. by [@DanCardin](https://github.com/DanCardin) in https://github.com/charliermarsh/ruff/pull/3417
* Respect `--show-fixes` with `--fix-only` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3426
* When "Args" and "Parameters" are present, prefer NumPy style by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3430
* Remove empty line after RUF100 auto-fix by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3414
* Avoid removing un-aliased exceptions in `OSError`-aliased handlers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3451
* Use a hash to fingerprint GitLab CI output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3456
* Avoid respecting noqa directives when RUF100 is enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3469
* Output GitLab paths relative to `CI_PROJECT_DIR` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3475
* Implement an iterator for universal newlines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3454

## New Contributors
* [@sasanjac](https://github.com/sasanjac) made their first contribution in https://github.com/charliermarsh/ruff/pull/3292
* [@aacunningham](https://github.com/aacunningham) made their first contribution in https://github.com/charliermarsh/ruff/pull/3380
* [@orf](https://github.com/orf) made their first contribution in https://github.com/charliermarsh/ruff/pull/3389
* [@DanCardin](https://github.com/DanCardin) made their first contribution in https://github.com/charliermarsh/ruff/pull/3417
* [@AreamanM](https://github.com/AreamanM) made their first contribution in https://github.com/charliermarsh/ruff/pull/3405
* [@kyoto7250](https://github.com/kyoto7250) made their first contribution in https://github.com/charliermarsh/ruff/pull/3411
* [@latonis](https://github.com/latonis) made their first contribution in https://github.com/charliermarsh/ruff/pull/3449
* [@XuehaiPan](https://github.com/XuehaiPan) made their first contribution in https://github.com/charliermarsh/ruff/pull/3459
* [@CalumY](https://github.com/CalumY) made their first contribution in https://github.com/charliermarsh/ruff/pull/3461
* [@YDX-2147483647](https://github.com/YDX-2147483647) made their first contribution in https://github.com/charliermarsh/ruff/pull/3473

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.254...v0.0.255

[Changes][v0.0.255]


<a name="v0.0.254"></a>
# [v0.0.254](https://github.com/astral-sh/ruff/releases/tag/v0.0.254) - 04 Mar 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`pyupgrade`] Replace tuples with type union in isinstance or issubclass calls by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/3280
* [`flake8-pyi`] Add flake-pyi PYI033 "Do not use type comments in stubs" by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/3302
* [`flake8-pyi`] PYI006 bad version info comparison by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/3291
* [`pycodestyle`] feat(E251,E252): add rules by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3274
* [`pycodestyle`] feat(E211): add rule + autofix by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3313
* [`pycodestyle`] feat(e225,226,227,228): add rules by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3300
* [`pycodestyle`] feat(W191): add indentation_contains_tabs by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3249
* Redirect `RUF004` to `B026` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3293

### CLI
* Add a `--ignore-noqa` CLI flag to force-ignore noqa directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3296

### Settings
* Implement `property-decorators` configuration option for pydocstyle by [@staticssleever668](https://github.com/staticssleever668) in https://github.com/charliermarsh/ruff/pull/3311
* Always include `@classmethod` and `@staticmethod` in decorator lists by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3314
* Detect quote style ignoring docstrings by [@bz2](https://github.com/bz2) in https://github.com/charliermarsh/ruff/pull/3306

### Bug Fixes
* Deduplicate SIM116 errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3260
* Don't flag keyword-based logging format strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3261
* Avoid raising TRY200 violations within new scopes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3275
* Use expression span for yoda-conditions fixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3276
* Avoid PEP 585 rewrites when builtins are shadowed by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3286
* Ignore unused imports in ModuleNotFoundError blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3288
* Treat function type annotations within classes as runtime-required by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3312
* Extend `RET503` autofixes to "end of statement", including comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3324
* Handle empty NamedTuple and TypedDict conversions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3251
* Use `identifier_range` for a few more rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3254
* Match non-lowercase with S105 again by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/3258
* Abort when unable to fix relative imports past module root by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3319
* Use presence of convention-specific sections during docstring inference by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3325
* Treat callables within type definitions as default-non-types by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3329
* Explicitly put `Path(...)` in Pathlib violations by [@evanrittenhouse](https://github.com/evanrittenhouse) in https://github.com/charliermarsh/ruff/pull/3333

## New Contributors
* [@rouge8](https://github.com/rouge8) made their first contribution in https://github.com/charliermarsh/ruff/pull/3277
* [@staticssleever668](https://github.com/staticssleever668) made their first contribution in https://github.com/charliermarsh/ruff/pull/3311
* [@bz2](https://github.com/bz2) made their first contribution in https://github.com/charliermarsh/ruff/pull/3306
* [@evanrittenhouse](https://github.com/evanrittenhouse) made their first contribution in https://github.com/charliermarsh/ruff/pull/3333

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.253...v0.0.254

[Changes][v0.0.254]


<a name="v0.0.253"></a>
# [v0.0.253](https://github.com/astral-sh/ruff/releases/tag/v0.0.253) - 27 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at 386ca7c9a1bb7ebeb1457b605695c6a09e67092b -->

## What's Changed

### Rules
* [`pyupgrade`] Avoid rewriting any PEP 604 runtime annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3217
* [`pycodestyle`] Missing whitespace after keyword by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3225
* [`pycodestyle`] trailing-whitespace, blank-line-contains-whitespace (W291, W293) by [@mknaw](https://github.com/mknaw) in https://github.com/charliermarsh/ruff/pull/3122
* [`flake8-pyi`]: PYI009, PYI010, PYI021 by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/3230
* [`flake8-pyi`]: PYI011, PYI014 by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/3238
* [`flake8-django`] DJ003, DJ006, DJ007 by [@lkh42t](https://github.com/lkh42t) in https://github.com/charliermarsh/ruff/pull/3236
* [`pylint`] Implement pylint's `else-if-used` rule (`PLR5501`) by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/3231
* [`pylint`] W0603: global-statement by [@igozali](https://github.com/igozali) in https://github.com/charliermarsh/ruff/pull/3227
* [`flake8-pie`] Unnecessary list comprehension, with autofix (PIE802) by [@matthewlloyd](https://github.com/matthewlloyd) in https://github.com/charliermarsh/ruff/pull/3149

### Settings
* Allow ruff.toml file to be dot-prefixed (as .ruff.toml) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3221
* [`pydocstyle`]: Implement `ignore-decorators` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/3229

### Bug Fixes
* Avoid suggesting 'is' for constant literals by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3146
* Omit non-.py[i] files from module naming rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3153
* Bind star patterns in match statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3169
* Update RustPython to support *tuple annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3178
* Use `writeln` with --show-settings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3180
* Avoid boolean-trap rules for ConfigParser get() methods by [@monosans](https://github.com/monosans) in https://github.com/charliermarsh/ruff/pull/3209
* Avoid flagging logging-too-few-args with no arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3220
* Expand the range of the COM812 autofix to include the preceding token by [@matthewlloyd](https://github.com/matthewlloyd) in https://github.com/charliermarsh/ruff/pull/3241
* Avoid flagging Pylint logging rules with starred arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3244
* Avoid flagging unfixable `TypedDict` and `NamedTuple` definitions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3148
* Fix ExceptionGroup F821 false positive by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3167
* Avoid autofixing some PT violations when comments are present by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3198
* Exclude globsets for --show-settings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3201
* [`flake8-tidy-imports`] fix autofix for relative imports by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/3197
* Fix Markdown errors in docs by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3187
* Normalize treatment of aliased and unaliased imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3216
* Avoid EXE001 and EXE002 errors from stdin input by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3218
* [bandit]: Do not treat "passed" as "password" for `S105`/`S106`/`S107` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/3222

## New Contributors
* [@mknaw](https://github.com/mknaw) made their first contribution in https://github.com/charliermarsh/ruff/pull/3122
* [@monosans](https://github.com/monosans) made their first contribution in https://github.com/charliermarsh/ruff/pull/3209
* [@lkh42t](https://github.com/lkh42t) made their first contribution in https://github.com/charliermarsh/ruff/pull/3236
* [@igozali](https://github.com/igozali) made their first contribution in https://github.com/charliermarsh/ruff/pull/3227

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.252...v0.0.253

[Changes][v0.0.253]


<a name="v0.0.252"></a>
# [v0.0.252](https://github.com/astral-sh/ruff/releases/tag/v0.0.252) - 22 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`pylint`] `redefined-loop-name` (`W2901`) by [@matthewlloyd](https://github.com/matthewlloyd) in https://github.com/charliermarsh/ruff/pull/3022
* [`pylint`] `logging-too-many-args ` (`E1205`) by [@md384](https://github.com/md384) in https://github.com/charliermarsh/ruff/pull/3084
* [`pylint`] `logging-too-few-args ` (`E1206`) by [@md384](https://github.com/md384) in https://github.com/charliermarsh/ruff/pull/3084

### Bug Fixes
* Include file permissions in cache key by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3104
* Skip EXE001 and EXE002 rules on Windows by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3111
* Mark `typing.assert_never` as no return by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/3121
* Use file-specific quote for C408 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3128
* Avoid match statement misidentification in token rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3129
* Upgrade RustPython to handle trailing commas in map patterns by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3130
* Avoid useless-else-on-loop for break within match by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3136
* Fix isort `no-lines-before` preceded by an empty section by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/3139
* Support shell expansion for --config argument by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3107
* Fix =/== error in `ManualDictLookup` by [@Rupt](https://github.com/Rupt) in https://github.com/charliermarsh/ruff/pull/3117
* Include match in nested block check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3137
* Upgrade RustPython to match new flattened exports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3141

## New Contributors
* [@md384](https://github.com/md384) made their first contribution in https://github.com/charliermarsh/ruff/pull/3084
* [@Rupt](https://github.com/Rupt) made their first contribution in https://github.com/charliermarsh/ruff/pull/3117
* [@marijncv](https://github.com/marijncv) made their first contribution in https://github.com/charliermarsh/ruff/pull/3133

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.251...v0.0.252

[Changes][v0.0.252]


<a name="v0.0.251"></a>
# [v0.0.251](https://github.com/astral-sh/ruff/releases/tag/v0.0.251) - 21 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Bug Fixes
* Create bindings for `MatchAs` patterns by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3098
* Avoid prefer-list-builtin for lambdas with `*args` or `**kwargs` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3102

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.250...v0.0.251

[Changes][v0.0.251]


<a name="v0.0.250"></a>
# [v0.0.250](https://github.com/astral-sh/ruff/releases/tag/v0.0.250) - 21 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

**Ruff now supports all Python 3.10 and 3.11 language features**, including:

- Structural Pattern Patching (`match` statements) ([PEP 634](https://peps.python.org/pep-0634/#class-patterns))
- Exception Groups (`except*` statements) ([PEP 654](https://peps.python.org/pep-0654/))

### Rules
* [`flake8-bugbear`] Add B029 (`except-with-empty-tuple`) from flake8-bugbear by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3068
* [`flake8-bugbear`] Add B032 (`unintentional-type-annotation`) from flake8_bugbear by [@carlosmiei](https://github.com/carlosmiei) in https://github.com/charliermarsh/ruff/pull/3085
* [`tryceratops`]: Add TRY401 (`verbose-log-messages`) by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/3036
* [`flake8-simplify`]: Add SIM116 (`manual-dict-lookup`) by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2767

### Features
* Add support for TryStar by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3089
* Add support for structural pattern matching by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3047

### Bug Fixes
* [`flake8-pytest`] Use LibCST to fix chained assertions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3087
* [`flake8-boolean-trap`] Avoid boolean-trap rules for positional-only builtin calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3075
* [`flake8-boolean-trap`] Ignore setters in flake8-boolean-trap by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3092
* [`flake8-return`] Omit `while-True` loops from implicit return enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3076

## New Contributors
* [@carlosmiei](https://github.com/carlosmiei) made their first contribution in https://github.com/charliermarsh/ruff/pull/3068

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.249...v0.0.250

[Changes][v0.0.250]


<a name="v0.0.249"></a>
# [v0.0.249](https://github.com/astral-sh/ruff/releases/tag/v0.0.249) - 20 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at 4cfa350112a82fb631909fc555588f3da8ba5750 -->

## What's Changed

### Bug Fixes
* Relax constraints on pep8-naming module validation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3043
* Do not autofix `E731` in class bodies by [@JoshKarpel](https://github.com/JoshKarpel) in https://github.com/charliermarsh/ruff/pull/3050
* Avoid assert() to assert statement conversion in expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3062

## New Contributors
* [@matthewlloyd](https://github.com/matthewlloyd) made their first contribution in https://github.com/charliermarsh/ruff/pull/3048
* [@JoshKarpel](https://github.com/JoshKarpel) made their first contribution in https://github.com/charliermarsh/ruff/pull/3050

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.248...v0.0.249

[Changes][v0.0.249]


<a name="v0.0.248"></a>
# [v0.0.248](https://github.com/astral-sh/ruff/releases/tag/v0.0.248) - 19 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`numpy`] numpy-legacy-random by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2960
* [`pycodestyle`] autofix useless semicolons by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/3001
* [`pep8-naming`] Implement `flake8-module-naming` by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2855
* [`flake8-self`] Ignore namedtuple methods in flake8-self by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2998
* [`flake8-simplify`] Merge convert-loop-to-any & convert-loop-to-all to reimplemented-builtin by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2903
* [`ruff`] Add support for `ensure_future` for RUF006 by [@Lunarmagpie](https://github.com/Lunarmagpie) in https://github.com/charliermarsh/ruff/pull/2943
* [`pylint`] error when `__init__` returns a value by [@r3m0t](https://github.com/r3m0t) in https://github.com/charliermarsh/ruff/pull/3007
* [`flake8-pytest-style`] autofix for composite-assertion (PT018) by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2732
* [`flake8-tidy-imports`] extend autofix of relative imports by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2990

### Settings
* Add support for file-scoped `noqa` directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2978
* Add configuration option for C408 to allow dict calls with keyword arguments. by [@manueljacob](https://github.com/manueljacob) in https://github.com/charliermarsh/ruff/pull/2977
* feat(isort): Implement isort.force_to_top by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2877

### Bug Fixes
* Fix add-required-import with multi-line offsets by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2946
* Support positional messages in assertion rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3002
* Avoid false-positives for break in with by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/3032
* Avoid trying to fix implicit returns with control flow by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2962
* Handle non-from __future__ imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2974
* Enforce D403 on methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2992
* Avoid zero-indexed column for IOError by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2995
* Fix for F541 unescape f-string by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2971
* Avoid raising `B027` violations in `.pyi` files by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/3016

## New Contributors
* [@Lunarmagpie](https://github.com/Lunarmagpie) made their first contribution in https://github.com/charliermarsh/ruff/pull/2943
* [@manueljacob](https://github.com/manueljacob) made their first contribution in https://github.com/charliermarsh/ruff/pull/2966
* [@mwtoews](https://github.com/mwtoews) made their first contribution in https://github.com/charliermarsh/ruff/pull/2973
* [@ortem](https://github.com/ortem) made their first contribution in https://github.com/charliermarsh/ruff/pull/2976
* [@thatlittleboy](https://github.com/thatlittleboy) made their first contribution in https://github.com/charliermarsh/ruff/pull/3027
* [@r3m0t](https://github.com/r3m0t) made their first contribution in https://github.com/charliermarsh/ruff/pull/3007

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.247...v0.0.248

[Changes][v0.0.248]


<a name="v0.0.247"></a>
# [v0.0.247](https://github.com/astral-sh/ruff/releases/tag/v0.0.247) - 15 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules

* [`NPY001`] deprecated type aliases by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2810
* [`RUF006`] Implement `asyncio-dangling-task` to track `asyncio.create_task` calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2935

### CLI
* Implement shell autocompletion for rule codes by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2906

### Bug Fixes

* Handle multiple receiver decorators in receiver-decorator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2858
* Avoid false-positives with multi-byte characters in B005 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2862
* Avoid false-positives for runtime-types in type checking blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2863
* Avoid noqa removal upon unhandled syntax errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2864
* Ignore non-imperative-mood in Google docstring convention by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2900
* [`flake8-tidy-imports`] autofix relative imports by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2891
* Respect self as positional-only argument in annotation rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2927
* Apply nullable-model-string-field to all classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2928
* Deduplicate files provided on the command-line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2931
* Re-show --target-version on CLI interface by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2859
* Remove autofix for prefer-type-error by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2880
* Avoid unnecessary-else violations in `elif` branches by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2881
* Extend B904 to else branches by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2886
* Allow private accesses on current class by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2929

## New Contributors
* [@Jeremiah-England](https://github.com/Jeremiah-England) made their first contribution in https://github.com/charliermarsh/ruff/pull/2884
* [@Chris-May](https://github.com/Chris-May) made their first contribution in https://github.com/charliermarsh/ruff/pull/2896
* [@simon04](https://github.com/simon04) made their first contribution in https://github.com/charliermarsh/ruff/pull/2904

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.246...v0.0.247

[Changes][v0.0.247]


<a name="v0.0.246"></a>
# [v0.0.246](https://github.com/astral-sh/ruff/releases/tag/v0.0.246) - 13 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Breaking Changes
* Remove multiple-statements-on-one-line-def (E704) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2773

### Rules
* [`flake8-pyi`]: add rules for unrecognized platform check (PYI007, PYI008) by [@SigureMo](https://github.com/SigureMo) in https://github.com/charliermarsh/ruff/pull/2805
* [`flake8-simplify`]: combine-if-conditions by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2823
* [`flake8-django`] Implemented flake8-django plugin rules by [@konysko](https://github.com/konysko) in https://github.com/charliermarsh/ruff/pull/2586

### CLI
* Implement `config` subcommand by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2775
* Add rendering of rule markdown for terminal output by [@ngnpope](https://github.com/ngnpope) in https://github.com/charliermarsh/ruff/pull/2747
* Add `--show-fixes` flag to show applied fixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2707

### Settings
* implemented option lines-between-types for isort by [@PushUpek](https://github.com/PushUpek) in https://github.com/charliermarsh/ruff/pull/2762

### Autofix
* [`pyflakes`] Support unused variable removal in multi-assignment statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2786
* [`flake8-comprehensions`] autofix C414 and C417 + bugfix by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2693
* [`flake8-comprehensions`] bugfix for C413 autofix by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2804
* [`flake8-simplify`] Use smarter inversion for comparison checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2831
* [`flake8-comprehensions`] improve autofix for C401, C402 and C417 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2806

### Bug Fixes
* Ignore colon-after-lambda in compound statement rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2771
* Don't treat all future import accesses as non-runtime by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2774
* Avoid treating deferred string annotations as required-at-runtime by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2779
* Respect continuations in `noqa` enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2783
* Improve unused-variable autofixes for with statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2785
* Refactor generator to use Astor-derived precedence levels by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2798
* Allow private accesses on super calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2815
* Allow non-verbose raise when cause is present by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2816
* Avoid duplicates in if-with-same-arms by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2827
* Include package inference during --add-noqa command by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2832
* Allow compound statements of single ellipsis by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2837

## New Contributors
* [@trag1c](https://github.com/trag1c) made their first contribution in https://github.com/charliermarsh/ruff/pull/2757
* [@PushUpek](https://github.com/PushUpek) made their first contribution in https://github.com/charliermarsh/ruff/pull/2762
* [@konysko](https://github.com/konysko) made their first contribution in https://github.com/charliermarsh/ruff/pull/2586
* [@SigureMo](https://github.com/SigureMo) made their first contribution in https://github.com/charliermarsh/ruff/pull/2805

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.245...v0.0.246

[Changes][v0.0.246]


<a name="v0.0.245"></a>
# [v0.0.245](https://github.com/astral-sh/ruff/releases/tag/v0.0.245) - 11 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed
### Breaking Changes
* Remove public Rust API by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2709

### Rules
* [`flake8-pyi`]Add flake8-pyi with one rule by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/2682
* [`pylint`]: bad-string-format-type by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2572
* [`pylint`]: yield-in-init by [@tomecki](https://github.com/tomecki) in https://github.com/charliermarsh/ruff/pull/2716
* [`flake8-tidy-imports`] Implement autofix for relative imports (TID252) by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2739
* [`flake8-bandit`]: try-except-continue by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2674
* [`flake8-bandit`] Implement bandit's 'hardcoded-sql-expressions' S608 by [@mattoberle](https://github.com/mattoberle) in https://github.com/charliermarsh/ruff/pull/2698
* [`pycodestyle`] Implement compound-statements (E701, E702, E703, E704) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2680

### Settings
* feat(isort): Implement known-local-folder by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2657
* Suppress parse errors with explicit `# noqa: E999` directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2697

### Bug Fixes
* Support callable decorators in classmethod_decorators et al by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2675
* Avoid flagging typed exceptions in tuples by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2728
* Mark flake8-simplify rules as unfixable in non-fixable cases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2676
* Treat re-exported annotations as used-at-runtime by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2689
* Gate `Path.readlink()` behind Python 3.9+ guard by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2708
* Handle functions that never return in RET503 ([#2602](https://github.com/astral-sh/ruff/issues/2602)) by [@ppentchev](https://github.com/ppentchev) in https://github.com/charliermarsh/ruff/pull/2701
* Handle more functions that never return in RET503 by [@ngnpope](https://github.com/ngnpope) in https://github.com/charliermarsh/ruff/pull/2719
* Expand S110 and S112 ranges to include entire exception handler by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2729
* Relax conditions in bad-string-format-type by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2731
* Mark `__all__` members as used at end-of-scope by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2733
* Only validate `__all__` bindings for global scope by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2738
* Only trigger compound statements after select keywords by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2737
* Use `function_type::classify` for `yield-in-init` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2742
* Allow named unicodes in bidirectional escape check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2710
* Respect NO_COLOR flags in --show-source by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2750
* Flag private member accesses on calls et al by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2753

## New Contributors
* [@ngnpope](https://github.com/ngnpope) made their first contribution in https://github.com/charliermarsh/ruff/pull/2692
* [@ppentchev](https://github.com/ppentchev) made their first contribution in https://github.com/charliermarsh/ruff/pull/2701
* [@tomecki](https://github.com/tomecki) made their first contribution in https://github.com/charliermarsh/ruff/pull/2716

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.244...v0.0.245

[Changes][v0.0.245]


<a name="v0.0.244"></a>
# [v0.0.244](https://github.com/astral-sh/ruff/releases/tag/v0.0.244) - 08 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`pylint`]: bidirectional-unicode by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2589
* Implement whitespace-around-keywords (E271, E272, E273, E274) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2653
* Implement whitespace-before-comment (E261, E262, E265, E266) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2654

### Settings
* flake8-annotations: add ignore-fully-untyped by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2128
* Add `--exit-non-zero-on-fix` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2668

### Documentation
* Derive `explanation` method on Rule struct via rustdoc by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2642
* Create per-rule pages and link from README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2644
* Add documentation for flake8-quotes rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2650
* Disable autofix for flake8-print rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2651
* Add documentation for eradicate, flake8-import-conventions, and flake8-no-pep420 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2652
* doc: add documentation for TRY002 by [@nm-remarkable](https://github.com/nm-remarkable) in https://github.com/charliermarsh/ruff/pull/2655

### Bug Fixes
* Accommodate pos-only arguments when checking self name by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2626
* Avoid non-recursion in nested typing function calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2627
* Avoid no-unnecessary-dict-kwargs errors with reserved keywords by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2628
* Treat [@staticmethod](https://github.com/staticmethod) as higher-precedence than ABC by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2635
* Avoid boolean-trap errors in `__setitem__` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2636
* Skip ternary fixes for yields and awaits by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2637
* Ignore all non-`.py` wrt. implicit namespace package by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2640
* Avoid false-positive in chained type calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2663
* Treat annotated assignments in class and module scopes as runtime by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2667
* Accommodate multiple `@pytest.mark.parametrize` decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2662
* Use separate exit codes for fatal errors vs. lint errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2670

## New Contributors
* [@nm-remarkable](https://github.com/nm-remarkable) made their first contribution in https://github.com/charliermarsh/ruff/pull/2655

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.243...v0.0.244

[Changes][v0.0.244]


<a name="v0.0.243"></a>
# [v0.0.243](https://github.com/astral-sh/ruff/releases/tag/v0.0.243) - 07 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* Add flake8-pie single_starts_ends_with by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/2616

### Bug Fixes
* fix(pep8-naming): `typing.NamedTuple` and `typing.TypedDict` treatment by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2611
* Support `ignore-names` for all relevant pep8-naming rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2617
* Avoid removing quotes from runtime annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2614
* Support relative paths for typing-modules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2615


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.242...v0.0.243

[Changes][v0.0.243]


<a name="v0.0.242"></a>
# [v0.0.242](https://github.com/astral-sh/ruff/releases/tag/v0.0.242) - 06 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* Implement pylint's `too-many-branches` rule (`PLR0912`) by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/2550
* Implement pylint's `too-many-return-statements` rule (`PLR0911`) by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/2564
* [`pyupgrade`]: Removes quotes from annotations by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2431
* [`pylint`]: bad-str-strip-call by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2570
* Implement pycodestyle's logical line detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1130
* Enable autofix for unnecessary-paren-on-raise-exception by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2596
* Automatically remove empty type-checking blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2598

### Bug Fixes
* Avoid infinite renames for unused-loop-control-variable by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2581
* Ignore direct root-children in implicit-namespace-package by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2565
* Allow blank line before sticky-comment functions in docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2597
* Visit deferred assignments after deferred type annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2607
* Fix python module invocation by [@trottomv](https://github.com/trottomv) in https://github.com/charliermarsh/ruff/pull/2563
* Portably find ruff binary path from Python by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/2574
* fix(commented-out-code): `mypy` and `SPDX-License-Identifier` false positives by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2587

## New Contributors
* [@trottomv](https://github.com/trottomv) made their first contribution in https://github.com/charliermarsh/ruff/pull/2563
* [@MichaReiser](https://github.com/MichaReiser) made their first contribution in https://github.com/charliermarsh/ruff/pull/2088

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.241...v0.0.242

[Changes][v0.0.242]


<a name="v0.0.241"></a>
# [v0.0.241](https://github.com/astral-sh/ruff/releases/tag/v0.0.241) - 04 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Deprecates
* Soft-deprecate `update-check` by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2530

### Rules
* feat: add autofix for PLR0402 by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2504
* Allow F811 noqa declarations on containing import lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2553

### Features
* Mark fixable issues in printer output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2500
* Always report parse errors back to the user by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2505
* Notify user if autofix introduces syntax error by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2507
* Add `known-standard-library` for each Python version by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/2491

### Bug Fixes
* Avoid renaming unused loop variables with deferred usages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2509
* [`flake8-self`] Fix False Negative Issue on Rule `SLF001` by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2527
* Minor fixes to PLR0915 logic by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/2518
* Exit upon showing files with `--show-files` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2543
* Ignore direct source-children in `implicit-namespace-package` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2560
* Allow `list()` and `tuple()` calls in `__all__` assignments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2499
* Don't walk past project root when figuring out exclusion by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2471
* Avoid hang when detecting trailing comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2549

## New Contributors
* [@JacobCoffee](https://github.com/JacobCoffee) made their first contribution in https://github.com/charliermarsh/ruff/pull/2516
* [@Pierre-Sassoulas](https://github.com/Pierre-Sassoulas) made their first contribution in https://github.com/charliermarsh/ruff/pull/2559

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.240...v0.0.241

[Changes][v0.0.241]


<a name="v0.0.240"></a>
# [v0.0.240](https://github.com/astral-sh/ruff/releases/tag/v0.0.240) - 02 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`pyupgrade`]: Remove outdated `sys.version_info` blocks by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2099
* [`flake8-self`] Add Plugin and Rule `SLF001` by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2470
* [`pylint`] Implement pylint's `too-many-statements` rule (`PLR0915`) by [@chanman3388](https://github.com/chanman3388) in https://github.com/charliermarsh/ruff/pull/2445

### Settings
* [`isort`] Support forced_separate by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2268
* [`isort`] Add isort option lines-after-imports by [@reidswan](https://github.com/reidswan) in https://github.com/charliermarsh/ruff/pull/2440
* Allow non-ruff.toml-named files for --config by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2463

### Bug Fixes
* Trigger, but don't fix, SIM rules if comments are present by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2450
* Only avoid PEP604 rewrites for pre-Python 3.10 code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2449
* Use LibCST to reverse Yoda conditions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2468
* fix: assertTrue()/assertFalse() fixer should not test for identity by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2476
* Treat `if 0:` and `if False:` as type-checking blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2485
* Avoid iterating over body twice by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2439
* more builtin name checks when autofixing by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2430
* Respect parent noqa in --add-noqa by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2464
* Avoid removing un-selected codes when applying `--add-noqa` edits by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2465
* Carry-over `ignore` to next config layer if `select = []` by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2467
* Visit NamedExpr values before targets by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2484

## New Contributors
* [@reidswan](https://github.com/reidswan) made their first contribution in https://github.com/charliermarsh/ruff/pull/2440
* [@chanman3388](https://github.com/chanman3388) made their first contribution in https://github.com/charliermarsh/ruff/pull/2445

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.239...v0.0.240

[Changes][v0.0.240]


<a name="v0.0.239"></a>
# [v0.0.239](https://github.com/astral-sh/ruff/releases/tag/v0.0.239) - 01 Feb 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### Rules
* [`pyupgrade`] Implement import-replacement rule (`UP035`) by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/2049
* [`flake8-raise`] Add Plugin and `RSE102` Rule by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2354
* [`flake8-quotes`] Allow implicit multiline strings with internal quotes to use non-preferred quote by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2416

### Documentation
* Add a link to MkDocs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2370
* Use human-readable types for documentation values by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2375

### Bug Fixes
* [`I001`] fix isort check for files with tabs and no indented blocks by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2374
* Don't panic for --statistics with no errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2391
* Handle multi-byte lines in RUF100 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2392
* Avoid implicit-namespace-package checks for .pyi files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2420
* Include per-file ignore matches in debug logging by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2376
* Include method name in B027 message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2403
* Avoid flagging same-condition cases in SIM103 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2404
* feat: let SIM103 return expressions without bool() wrapping by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2410
* feat: let SIM210 return expressions without bool() wrapping ([#2410](https://github.com/astral-sh/ruff/issues/2410)) by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2426
* fix: ignore fix if "bool" is not builtin by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2429
* Avoid Bandit false-positives for empty-string-as-password by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2421

## New Contributors
* [@eriknw](https://github.com/eriknw) made their first contribution in https://github.com/charliermarsh/ruff/pull/2380
* [@has2k1](https://github.com/has2k1) made their first contribution in https://github.com/charliermarsh/ruff/pull/2386

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.238...v0.0.239

[Changes][v0.0.239]


<a name="v0.0.238"></a>
# [v0.0.238](https://github.com/astral-sh/ruff/releases/tag/v0.0.238) - 30 Jan 2023

<!-- Release notes generated using configuration in .github/release.yml at main -->

## What's Changed

### ⚠️ Breaking Changes ⚠️ 

#### `select`, `extend-select`, `ignore`, and `extend-ignore` have new semantics ([#2312](https://github.com/charliermarsh/ruff/pull/2312))

Previously, the interplay between `select` and its related options could lead to unexpected behavior. For example, `ruff --select E501 --ignore ALL` and `ruff --select E501 --extend-ignore ALL` behaved differently. (See [#2312](https://github.com/charliermarsh/ruff/pull/2312) for more examples.)

The new semantics are such that Ruff uses the "highest-priority" `select` as the basis for the rule set, and then applies any `extend-select`, `ignore`, and `extend-ignore` adjustments. CLI options are given higher priority than `pyproject.toml` options, and the current `pyproject.toml` file is given higher priority than any inherited `pyproject.toml` files.

As an example: `ruff --select F401` will select rule `F401`, and ignore any of the modifiers from the `pyproject.toml`, as the "highest-priorty" select kicks off the resolution chain.

This change is largely backwards compatible -- most users should experience no change in behavior. For more, see [BREAKING_CHANGES.md](https://github.com/charliermarsh/ruff/blob/main/BREAKING_CHANGES.md#select-extend-select-ignore-and-extend-ignore-have-new-semantics-2312).

#### `remove-six-compat` (`UP016`) has been removed ([#2332](https://github.com/charliermarsh/ruff/pull/2332))

The `remove-six-compat` rule has been removed. This rule was only useful for one-time Python 2-to-3 upgrades.

### Rules
* Implement Pylint's `too-many-arguments` rule (`PLR0913`) by [@akhildevelops](https://github.com/akhildevelops) in https://github.com/charliermarsh/ruff/pull/2308
* Extend conventional imports defaults to include TensorFlow et al by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2353

### Settings
* feat: add ruff --statistics by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2284
* Ignore magic comparisons to bytes by default by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2365
* Implement `ruff linter` subcommand by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2294
* Improve rule config resolution by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2312

### Bug Fixes
* Refine criteria for `exc_info` logger rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2364
* Respect per-file-ignores when checking noqa by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2309
* Place star before other member imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2320
* Allow list comprehensions for __all__ assignment by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2326
* [`TRY201`] don't check raise statements in nested exception handlers by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2337
* include tomllib in standard lib by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2345
* Avoid removing trailing comments when autofixing by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2352
* [`I001`] fix isort for files with tab-based indentation by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2361
* Disable incompatible rules rather than merely warning by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2369

## New Contributors
* [@chirag127](https://github.com/chirag127) made their first contribution in https://github.com/charliermarsh/ruff/pull/2307
* [@akhildevelops](https://github.com/akhildevelops) made their first contribution in https://github.com/charliermarsh/ruff/pull/2308

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.237...v0.0.238

[Changes][v0.0.238]


<a name="v0.0.237"></a>
# [v0.0.237](https://github.com/astral-sh/ruff/releases/tag/v0.0.237) - 28 Jan 2023

## What's Changed


### Breaking Changes

`--explain`, `--clean`, and `--generate-shell-completion` are now implemented as subcommands ([#2190](https://github.com/charliermarsh/ruff/pull/2190)):

    ruff .         # Still works! And will always work.
    ruff check .   # New! Also works.

    ruff --explain E402   # Still works.
    ruff rule E402        # New! Also works. (And preferred.)

    # Oops! The command has to come first.
    ruff --format json --explain E402   # No longer works.
    ruff --explain E402 --format json   # Still works!
    ruff rule E402   --format json      # Works! (And preferred.)

This change is largely backwards compatible -- most users should experience no change in behavior. For exceptions, see [`BREAKING_CHANGES.md`](https://github.com/charliermarsh/ruff/blob/main/BREAKING_CHANGES.md#--explain---clean-and---generate-shell-completion-are-now-subcommands-2190).

### Rules
* feat: pylint `PLE0604` and `PLE0605` by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2241
* feat: include os.getcwdb (bytes) into flake8-use-pathlib by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2276
* [`flake8-bandit`] Add Rule S110 (try/except/pass) by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2197

### Settings
* Omit typing module from flake8-type-checking by default by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2277

### Bug Fixes
* Fix SIM300 to take Python constants into account by [@frenck](https://github.com/frenck) in https://github.com/charliermarsh/ruff/pull/2255
* Treat constant tuples as constants for yoda-conditions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2265
* Treat attribute constants as constant for yoda-conditions by [@frenck](https://github.com/frenck) in https://github.com/charliermarsh/ruff/pull/2266
* flake8-annotations: Move has_any_typed_arg into correct nested `if` by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2269
* Expand heuristic for detecting logging calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2279
* Preserve global binding kind during reassignments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2297
* Fix typo in typing_extensions by [@Jonxslays](https://github.com/Jonxslays) in https://github.com/charliermarsh/ruff/pull/2298

## New Contributors
* [@frenck](https://github.com/frenck) made their first contribution in https://github.com/charliermarsh/ruff/pull/2255
* [@sladyn98](https://github.com/sladyn98) made their first contribution in https://github.com/charliermarsh/ruff/pull/2141
* [@the-matt-morris](https://github.com/the-matt-morris) made their first contribution in https://github.com/charliermarsh/ruff/pull/2291
* [@Jonxslays](https://github.com/Jonxslays) made their first contribution in https://github.com/charliermarsh/ruff/pull/2298

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.236...v0.0.237

[Changes][v0.0.237]


<a name="v0.0.236"></a>
# [v0.0.236](https://github.com/astral-sh/ruff/releases/tag/v0.0.236) - 26 Jan 2023

## What's Changed

### Rules
* feat: implement `TRY002` and `TRY003` by [@karpa4o4](https://github.com/karpa4o4) in https://github.com/charliermarsh/ruff/pull/2135
* Implementing `TRY400` by [@Flowake](https://github.com/Flowake) in https://github.com/charliermarsh/ruff/pull/2115
* Implement some rules from `flake8-logging-format` (`G`) by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/2150

### Settings
* Add strictness setting for `flake8-typing-imports` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2221
* Implement `exempt-modules` setting from flake8-type-checking by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2230

### Bug Fixes

* flake8_executable: Only match shebang at beginning of line by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/2183
* Don't flag B009/B010 if identifiers would be mangled by [@sciyoshi](https://github.com/sciyoshi) in https://github.com/charliermarsh/ruff/pull/2204
* fix: --explain reporting the wrong linter by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2215
* Preserve indentation when fixing via LibCST by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2223
* Avoid erroneous class autofixes in indented blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2226
* Fix range for `try-consider-else` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2228
* Avoid flagging blind exceptions with valid logging by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2232
* Avoid removing trailing comments on `pass` statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2235
* Allow `pytest` in shebang by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2237
* Wrap return-bool-condition-directly fixes in bool() by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2240

## New Contributors
* [@Flowake](https://github.com/Flowake) made their first contribution in https://github.com/charliermarsh/ruff/pull/2115
* [@henryiii](https://github.com/henryiii) made their first contribution in https://github.com/charliermarsh/ruff/pull/2200
* [@sciyoshi](https://github.com/sciyoshi) made their first contribution in https://github.com/charliermarsh/ruff/pull/2204

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.235...v0.0.236

[Changes][v0.0.236]


<a name="v0.0.235"></a>
# [v0.0.235](https://github.com/astral-sh/ruff/releases/tag/v0.0.235) - 25 Jan 2023

## Summary

Follow-up release to `v0.0.234` to fix two non-fatal issues related to CLI output.

(No new rules or functionality.)

## What's Changed
* Avoid duplicate CI runs triggered by pushes to pull requests by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/2178
* Restore single-file license by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2180
* Windows compatibility by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2033
* Remove stray parenthesis from fixed errors message by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/2181
* Fix conflicting error message warning by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2182


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.234...v0.0.235

[Changes][v0.0.235]


<a name="v0.0.234"></a>
# [v0.0.234](https://github.com/astral-sh/ruff/releases/tag/v0.0.234) - 25 Jan 2023

## What's Changed
* Move is_overlong to helpers by [@ericroberts](https://github.com/ericroberts) in https://github.com/charliermarsh/ruff/pull/2137
* Update .pre-commit-config.yml by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/2139
* Avoid generating dirty call paths by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2144
* Implement runtime-import-in-type-checking-block (TYP004) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2146
* Implement typing-only import detection (TYP001, TYP002, TYP003) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2147
* Add `#![warn(clippy::pedantic)]` to lib.rs and main.rs files by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2148
* Treat Python 3.7 as minimum supported version by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2159
* Avoid flagging unused loop variable (B007) with locals() by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2161
* Avoid prefer-type-error (TRY004) with intermediary control flow by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2162
* Add an FAQ on autofix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2163
* Avoid re-resolving settings for repeated paths by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2165
* Suggest input format in error case by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2167
* Re-add error wrapper in main.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2168
* fix: avoid flagging unused loop variable (B007) with globals(), vars() or eval() by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2166
* Fix singular and plural for "error(s)" by [@hugovk](https://github.com/hugovk) in https://github.com/charliermarsh/ruff/pull/2157
* Add Babel to readme by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2170
* Add flake8-builtins options to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2173
* Avoid reraise-no-cause for explicit reraises by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2174
* Rename TYP rules to TYC by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2175
* Actually, rename TYP rules to TCH by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2176


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.233...v0.0.234

[Changes][v0.0.234]


<a name="v0.0.233"></a>
# [v0.0.233](https://github.com/astral-sh/ruff/releases/tag/v0.0.233) - 24 Jan 2023

This is a rerun of `v0.0.232` (unreleased) to address build failures on Windows.

## What's Changed
* Move compare to helpers file by [@ericroberts](https://github.com/ericroberts) in https://github.com/charliermarsh/ruff/pull/2131
* Enable executable checks on Windows by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2133


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.232...v0.0.233

[Changes][v0.0.233]


<a name="v0.0.232"></a>
# [v0.0.232](https://github.com/astral-sh/ruff/releases/tag/v0.0.232) - 24 Jan 2023

## What's Changed
* Escape curly braces when converting .format() strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2112
* feat: implement TRY200 by [@karpa4o4](https://github.com/karpa4o4) in https://github.com/charliermarsh/ruff/pull/2087
* Upgrade to toml v0.6.0 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2116
* Allow flagging of multiline implicit string concatenations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2117
* feat: implement TRY301 by [@karpa4o4](https://github.com/karpa4o4) in https://github.com/charliermarsh/ruff/pull/2113
* Add Home Assistant to Readme by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2120
* Add apk instructions to README by [@WhyNotHugo](https://github.com/WhyNotHugo) in https://github.com/charliermarsh/ruff/pull/2121
* Some refactors by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2122
* Implement `EXE001` and `EXE002` from `flake8-executable` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/2118
* flake8-annotations: deduplicate code between functions and methods by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2125
* Ignore generators in flake8-return rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2126
* feat: autofix `multi-line-summary-*-line` by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2093
* Update flake8-to-ruff to include latest plugins by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2127
* refactor: Move redirects out of RuleCodePrefix  by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2103
* Move pycodestyle rules into individual files by [@ericroberts](https://github.com/ericroberts) in https://github.com/charliermarsh/ruff/pull/2123
* Remove unnecessary manual Generator invocations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2129
* Add Dagger and Great Expectations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2130

## New Contributors
* [@WhyNotHugo](https://github.com/WhyNotHugo) made their first contribution in https://github.com/charliermarsh/ruff/pull/2121

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.231...v0.0.232

[Changes][v0.0.232]


<a name="v0.0.231"></a>
# [v0.0.231](https://github.com/astral-sh/ruff/releases/tag/v0.0.231) - 23 Jan 2023

## What's Changed
* fix: issue D401 only for non-test/property functions and methods by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2071
* feat: flake8-use-pathlib PTH100-124 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2090
* refactor: remove redundant enum by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2091
* feat: Implement TRY201 by [@alonme](https://github.com/alonme) in https://github.com/charliermarsh/ruff/pull/2073
* Avoid nested-if violations when outer-if has else clause by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2095
* Add flake8-pie PIE804: no-unnecessary-dict-kwargs by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/1884
* Add flake8-pie PIE800: no-unnecessary-spread by [@sbdchd](https://github.com/sbdchd) in https://github.com/charliermarsh/ruff/pull/1881
* Remove some usages of default format for expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2100
* docs(readme): add pypa cibuildwheel by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2107
* refactor: Get rid of `build.rs` and other refactors by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2101
* Fix outdated description of ruff's support of isort settings by [@thomkeh](https://github.com/thomkeh) in https://github.com/charliermarsh/ruff/pull/2106
* [`flake8-bandit`] Added Rule `S612` (Use of insecure `logging.config.listen`) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2108

## New Contributors
* [@sbdchd](https://github.com/sbdchd) made their first contribution in https://github.com/charliermarsh/ruff/pull/1884

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.230...v0.0.231

[Changes][v0.0.231]


<a name="v0.0.230"></a>
# [v0.0.230](https://github.com/astral-sh/ruff/releases/tag/v0.0.230) - 22 Jan 2023

## What's Changed
* fix: pin rustpython to the same revision to fix cargo vendor by [@figsoda](https://github.com/figsoda) in https://github.com/charliermarsh/ruff/pull/2069
* feat: implementation for TRY004 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2066
* ICN001 import-alias-is-not-conventional should check "from" imports by [@Zeddicus414](https://github.com/Zeddicus414) in https://github.com/charliermarsh/ruff/pull/2070
* Update link to Pylint parity tracking issue by [@cosmojg](https://github.com/cosmojg) in https://github.com/charliermarsh/ruff/pull/2074
* ICN001 check from imports that have no alias by [@Zeddicus414](https://github.com/Zeddicus414) in https://github.com/charliermarsh/ruff/pull/2072
* Index source code upfront to power (row, column) lookups by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1990
* Remove remaining `ropey` usages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2076
* Include package path in cache key by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2077
* feat: update scripts to new rules structure by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2078
* Base `INP` check on package inference by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2079
* Improve generator precedence operations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2080
* Support decorators in source code generator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2081
* feat: enable autofix for TRY004 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2084
* Refactor, decouple and support "PL" by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2051
* [`pep8-naming`][`N806`] Don't mark `TypeVar` & `NewType` Assignment as Errors by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2085
* Update linters pypi links to latest version by [@alonme](https://github.com/alonme) in https://github.com/charliermarsh/ruff/pull/2062
* flake8_to_ruff: support `isort` options by [@shannonrothe](https://github.com/shannonrothe) in https://github.com/charliermarsh/ruff/pull/2082
* Update RustPython to fix `Dict.keys` type by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/2086

## New Contributors
* [@figsoda](https://github.com/figsoda) made their first contribution in https://github.com/charliermarsh/ruff/pull/2069
* [@cosmojg](https://github.com/cosmojg) made their first contribution in https://github.com/charliermarsh/ruff/pull/2074
* [@alonme](https://github.com/alonme) made their first contribution in https://github.com/charliermarsh/ruff/pull/2062
* [@shannonrothe](https://github.com/shannonrothe) made their first contribution in https://github.com/charliermarsh/ruff/pull/2082

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.229...v0.0.230

[Changes][v0.0.230]


<a name="v0.0.229"></a>
# [v0.0.229](https://github.com/astral-sh/ruff/releases/tag/v0.0.229) - 21 Jan 2023

## What's Changed
* README: `--force-exclude` is already set by [@hugovk](https://github.com/hugovk) in https://github.com/charliermarsh/ruff/pull/2042
* Upgrade to toml v0.5.11 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2040
* Add support for pycodestyle E101 by [@ericroberts](https://github.com/ericroberts) in https://github.com/charliermarsh/ruff/pull/2038
* [`flake8-executable`] EXE003-005 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2023
* perf: Reduce allocations by [@Stranger6667](https://github.com/Stranger6667) in https://github.com/charliermarsh/ruff/pull/2045
* refactor: RuleOrigin, RuleCodePrefix and Rule::origin by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2029
* Add scaffolding for `flake8-type-checking` extension by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2048
* De-duplicate SIM102 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/2050
* Fix S101 range to only highlight `assert` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/2052
* Avoid removing comments in RUF005 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2057
* Revert "Upgrade to toml v0.5.11" by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2058
* Pyupgrade: Printf string formatting by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1803
* [`flake8-builtins`] Add `builtins-ignorelist` Option by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/2061
* feat: plugin scaffold for tryceratops with TRY300 by [@sbrugman](https://github.com/sbrugman) in https://github.com/charliermarsh/ruff/pull/2055
* Avoid flagging redefined imports as unused in same-scope by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2065

## New Contributors
* [@hugovk](https://github.com/hugovk) made their first contribution in https://github.com/charliermarsh/ruff/pull/2042
* [@ericroberts](https://github.com/ericroberts) made their first contribution in https://github.com/charliermarsh/ruff/pull/2038
* [@sbrugman](https://github.com/sbrugman) made their first contribution in https://github.com/charliermarsh/ruff/pull/2023

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.228...v0.0.229

[Changes][v0.0.229]


<a name="v0.0.228"></a>
# [v0.0.228](https://github.com/astral-sh/ruff/releases/tag/v0.0.228) - 20 Jan 2023

## What's Changed
* Pyupgrade: Extraneous parenthesis by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1926
* Port pydocstyle code 401 (ImperativeMood) by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/1999
* Change CI to use MSRV for cargo test and build by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2019
* Note `.astimezone()` in call-datetime-strptime-without-zone message by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2015
* Avoid trimming docstring if ends in trailing quote by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2025
* Grammar fixes by [@scop](https://github.com/scop) in https://github.com/charliermarsh/ruff/pull/2014
* Refactor and update `scripts/add_*.py` by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2016
* Use platform-appropriate newline character for LibCST embedding by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2028
* Bump terminfo to remove a whole bunch of unnecessary dependencies by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2022
* Move readme dev details to CONTRIBUTING.md and fix contradictions by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2030
* fix(pydocstyle): Avoid trimming docstring if starts with leading quote by [@spaceone](https://github.com/spaceone) in https://github.com/charliermarsh/ruff/pull/2027
* Fix D404 NoThisPrefix not working with whitespace. by [@Zeddicus414](https://github.com/Zeddicus414) in https://github.com/charliermarsh/ruff/pull/2036
* Only fix true-false returns for return-bool-condition-directly by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2037

## New Contributors
* [@scop](https://github.com/scop) made their first contribution in https://github.com/charliermarsh/ruff/pull/2015
* [@spaceone](https://github.com/spaceone) made their first contribution in https://github.com/charliermarsh/ruff/pull/2027
* [@Zeddicus414](https://github.com/Zeddicus414) made their first contribution in https://github.com/charliermarsh/ruff/pull/2036

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.227...v0.0.228

[Changes][v0.0.228]


<a name="v0.0.227"></a>
# [v0.0.227](https://github.com/astral-sh/ruff/releases/tag/v0.0.227) - 20 Jan 2023

## What's Changed
* Drop `RuleCode` in favor of `Rule` enum with human-friendly names by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1941
* Make define_rule_mapping! set rule code as doc comment of variants by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1991
* Added pylint formatter by [@damienallen](https://github.com/damienallen) in https://github.com/charliermarsh/ruff/pull/1995
* Preserve unmatched comparators in `SIM109` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1998
* Drop `Violation::placeholder` by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1996
* Apply #[derive(Default)] fixes suggested by Clippy by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2000
* Avoid `SIM201` and `SIM202` errors in `__ne__` et al by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2001
* Fix that --explain panics by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2002
* Split up pydocstyle rules by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/2003
* Add RUF005 "unpack instead of concatenating" check by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/1957
* Enable autofix for `FitsOnOneLine` (`D200`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2006
* Change AsRef<str> impl for Rule to kebab-case by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2009
* Upgrade RustPython by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2011
* Avoid SIM401 in `elif` blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2012
* Improve --explain output by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/2010
* Avoid checking row types for single-name [@parametrize](https://github.com/parametrize) decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/2013

## New Contributors
* [@damienallen](https://github.com/damienallen) made their first contribution in https://github.com/charliermarsh/ruff/pull/1995

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.226...v0.0.227

[Changes][v0.0.227]


<a name="v0.0.226"></a>
# [v0.0.226](https://github.com/astral-sh/ruff/releases/tag/v0.0.226) - 19 Jan 2023

## What's Changed
* [`isort`] Add `constants` and `variables` Options by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1951
* Fix bad link for flake8-no-pep420 by [@skykasko](https://github.com/skykasko) in https://github.com/charliermarsh/ruff/pull/1952
* Autofix SIM102 (NestedIfStatements) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1657
* Confine type-of-primitive checks to builtin type calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1962
* Autofix SIM117 (MultipleWithStatements) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1961
* [`isort`] Add `no-lines-before` Option by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1955
* Use `smallvec` for call path representation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1960
* Treat subscript accesses as unsafe effects for autofix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1966
* Strip whitespace when injecting D209 newline by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1967
* README: Link “Flake8” for consistency with the rest of the list by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1969
* Run cargo fmt in pre-commit by [@akx](https://github.com/akx) in https://github.com/charliermarsh/ruff/pull/1968
* Convert remaining call path sites to use `SmallVec` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1972
* Remove artificial wraps from GitHub messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1977
* Invert order of yoda-conditions message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1979
* Replace misplaced-comparison-constant with SIM300 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1980
* Use relative paths for INP001 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1981
* Avoid removing side effects for boolean simplifications by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1984
* Enable suppression of magic values by type by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1987
* Exclude None, Bool, and Ellipsis from ConstantType by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1988

## New Contributors
* [@skykasko](https://github.com/skykasko) made their first contribution in https://github.com/charliermarsh/ruff/pull/1952
* [@akx](https://github.com/akx) made their first contribution in https://github.com/charliermarsh/ruff/pull/1968

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.225...v0.0.226

[Changes][v0.0.226]


<a name="v0.0.225"></a>
# [v0.0.225](https://github.com/astral-sh/ruff/releases/tag/v0.0.225) - 18 Jan 2023

## What's Changed
* Define origin names & URLs within doc comments by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1929
* Refactor settings by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1930
* Allow duplicate enum values for enum.auto() by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1933
* Move `@functools.cache` rewrites to their own rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1938
* cli: Catch panics to tell the user to report them by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1928
* Avoid autofixing comma rules when --fix is not set by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1940
* Avoid broken autofix for `SIM103` with `elif` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1944
* Implement `flake8-no-pep420` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1942
* Exempt `contextlib.ExitStack()` for SIM115 rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1946
* Restrict SIM105 to try blocks with a body of one simple statement by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1948


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.224...v0.0.225

[Changes][v0.0.225]


<a name="v0.0.224"></a>
# [v0.0.224](https://github.com/astral-sh/ruff/releases/tag/v0.0.224) - 17 Jan 2023

## What's Changed
* Re-run benchmark and update documentation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1907
* Derive Hash instead of implementing it by hand by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1890
* Add backticks to B904's message by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1914
* Refactor `flake8_tidy_imports` by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1909
* Trigger update to pre-commit mirror after pypi publish by [@pmbarrett314](https://github.com/pmbarrett314) in https://github.com/charliermarsh/ruff/pull/1910
* Rewrite `lru_cache` to `cache` on Python 3.9+ by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1918
* Avoid syntax errors when fixing parenthesized unused variables by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1919
* Add some new testimonials by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1921
* Avoid removing statements that contain side-effects by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1920
* Add benchmark scripts for no-IO by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1925
* Add flake8-pie PIE796: prefer-unique-enum by [@ljesparis](https://github.com/ljesparis) in https://github.com/charliermarsh/ruff/pull/1923
* [pyupgrade] Automatically rewrite format-strings to f-strings by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1905

## New Contributors
* [@pmbarrett314](https://github.com/pmbarrett314) made their first contribution in https://github.com/charliermarsh/ruff/pull/1910
* [@ljesparis](https://github.com/ljesparis) made their first contribution in https://github.com/charliermarsh/ruff/pull/1923

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.223...v0.0.224

[Changes][v0.0.224]


<a name="v0.0.223"></a>
# [v0.0.223](https://github.com/astral-sh/ruff/releases/tag/v0.0.223) - 16 Jan 2023

## What's Changed
* Turn define_rule_mapping! into a procedural macro by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1885
* Convert confusable violations to named fields by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1887
* Add a dedicated token indexer for continuations and comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1886
* Remove some Clippy allows by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1888
* Update add plugin/rule scripts by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1889
* Improve magic value message wording by [@TomFryers](https://github.com/TomFryers) in https://github.com/charliermarsh/ruff/pull/1892
* Use more precise error ranges for RET505~508 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1895
* Implement flake8-commas by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/1872
* refactor: Split CliSettings from Settings by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1891
* Skip noqa checker if no diagnostics are found by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1898
* Don't require docstrings for setters and deleters by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1899
* Buffer diagnostic writes to `stdout` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1900
* Lock `stdout` once when printing diagnostics by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1901
* Avoid triggering SIM117 for async with statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1903

## New Contributors
* [@TomFryers](https://github.com/TomFryers) made their first contribution in https://github.com/charliermarsh/ruff/pull/1892

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.222...v0.0.223

[Changes][v0.0.223]


<a name="v0.0.222"></a>
# [v0.0.222](https://github.com/astral-sh/ruff/releases/tag/v0.0.222) - 15 Jan 2023

## What's Changed
* Add support for namespace packages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1859
* Improve `SIM117` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1867
* Bump RustPython by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/1836
* Split up the table corresponding to the pylint rules by [@thomkeh](https://github.com/thomkeh) in https://github.com/charliermarsh/ruff/pull/1868
* Reduce APIs and add top-level doc comments by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1866
* Add Dagster and SnowCLI by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1870
* Introduce ruff::rules module by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1865
* Make some internal APIs private by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1874
* Remove --max-complexity from the CLI by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1877
* Fix range of SIM201, 202, and 208 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1880
* Make the CI check for broken links in the Rust docs by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1883
* Turn doc references into links by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1878

## New Contributors
* [@thomkeh](https://github.com/thomkeh) made their first contribution in https://github.com/charliermarsh/ruff/pull/1868

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.221...v0.0.222

[Changes][v0.0.222]


<a name="v0.0.221"></a>
# [v0.0.221](https://github.com/astral-sh/ruff/releases/tag/v0.0.221) - 14 Jan 2023

## What's Changed
* Document the way extend-ignore/select are applied by [@jankatins](https://github.com/jankatins) in https://github.com/charliermarsh/ruff/pull/1839
* Implement `PLR2004` (`MagicValueComparison`) by [@max0x53](https://github.com/max0x53) in https://github.com/charliermarsh/ruff/pull/1828
* Use absolute paths for --stdin-filename matching by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1843
* [`flake8-bugbear`] Fix False Positives for `B024` & `B027` by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1851
* Clarify that some flake8-bugbear opinionated rules are already implemented by [@nsoranzo](https://github.com/nsoranzo) in https://github.com/charliermarsh/ruff/pull/1847
* [`isort`] Add `classes` Config Option by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1849
* Implement `PLR0133` (`ComparisonOfConstants`) by [@max0x53](https://github.com/max0x53) in https://github.com/charliermarsh/ruff/pull/1841
* Remove non-magic trailing comma from tuple by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1854
* Improve spacing preservation for `C405` fixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1855
* Refactor import-tracking to leverage existing AST bindings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1856
* Split off ruff_cli crate from ruff library by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1816
* Added ALE by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1857
* Add workaround for wasm-pack bug to fix the playground CI by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1861
* Actually fix wasm-pack build command by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1862
* Avoid unnecessary allocations for module names by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1863

## New Contributors
* [@jankatins](https://github.com/jankatins) made their first contribution in https://github.com/charliermarsh/ruff/pull/1839
* [@max0x53](https://github.com/max0x53) made their first contribution in https://github.com/charliermarsh/ruff/pull/1828
* [@nsoranzo](https://github.com/nsoranzo) made their first contribution in https://github.com/charliermarsh/ruff/pull/1847

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.220...v0.0.221

[Changes][v0.0.221]


<a name="v0.0.220"></a>
# [v0.0.220](https://github.com/astral-sh/ruff/releases/tag/v0.0.220) - 12 Jan 2023

## What's Changed
* Modify visibility and shuffle around some modules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1807
* Add usage of ruff in pandas to README by [@BioGeek](https://github.com/BioGeek) in https://github.com/charliermarsh/ruff/pull/1811
* [`flake8-bandit`] Add Rule for `S701` (jinja2 autoescape false) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1815
* Implement autofix for flake8-quotes by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1810
* Don't trigger SIM401 for complex default values by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1825
* Decouple by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1822
* Avoid parsing pyproject.toml files when settings are fixed by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1827
* 📝 Update readme example for adding isort required imports by [@nefrob](https://github.com/nefrob) in https://github.com/charliermarsh/ruff/pull/1824
* Implement isort's `reverse_relative` setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1826
* Avoid SIM110/SIM110 errors with else statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1832
* Airflow is now using ruff by [@ashb](https://github.com/ashb) in https://github.com/charliermarsh/ruff/pull/1833
* Support for-else loops in `SIM110` and `SIM111` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1834
* Use absolute paths for GitHub and Gitlab annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1837

## New Contributors
* [@BioGeek](https://github.com/BioGeek) made their first contribution in https://github.com/charliermarsh/ruff/pull/1811
* [@nefrob](https://github.com/nefrob) made their first contribution in https://github.com/charliermarsh/ruff/pull/1824
* [@ashb](https://github.com/ashb) made their first contribution in https://github.com/charliermarsh/ruff/pull/1833

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.219...v0.0.220

[Changes][v0.0.220]


<a name="v0.0.219"></a>
# [v0.0.219](https://github.com/astral-sh/ruff/releases/tag/v0.0.219) - 12 Jan 2023

## What's Changed
* Disable update check by default by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1786
* Improve PIE794 autofix behavior by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1794
* Convert flake8-comprehensions checks to Checker style by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1795
* Refactor flake8-comprehensions rules to take fewer arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1797
* Avoid rewriting flake8-comprehensions expressions for builtin overrides by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1799
* Update readme to reflect [#1763](https://github.com/astral-sh/ruff/issues/1763) by [@Czaki](https://github.com/Czaki) in https://github.com/charliermarsh/ruff/pull/1780
* Avoid flagging builtins for OSError rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1800
* Skip unused argument checks for magic methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1801
* Skip SIM108 violations for complex if-statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1802
* [`flake8-simplify`] Add Rule for `SIM115` (Use context handler for opening files) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1782
* flake8_simplify : SIM401 by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/1778
* Avoid erroneous Q002 error message for single-quote docstrings by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1777
* Implement doc line length enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1804
* Move top level `ruff` into `python` folder by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1806
* Improve globset documentation and help message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1808

## New Contributors
* [@Czaki](https://github.com/Czaki) made their first contribution in https://github.com/charliermarsh/ruff/pull/1780

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.218...v0.0.219

[Changes][v0.0.219]


<a name="v0.0.218"></a>
# [v0.0.218](https://github.com/astral-sh/ruff/releases/tag/v0.0.218) - 11 Jan 2023

## What's Changed
* Implement flake8-simplify SIM112 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1764
* Do not autofix PT004 and PT005 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1763
* Disable release builds on CI by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1761
* Move CONTRIBUTING.md to top-level by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1768
* [`flake8-bandit`] Add Rule for `S508` (snmp insecure version) & `S509` (snmp weak cryptography) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1771
* Generate RuleCode::origin() via macro by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1770
* Disable doctests by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1772
* Enable isort-style `required-imports` enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1762
* Pyupgrade: Format specifiers by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1594
* Avoid B023 false-positives for some common builtins by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1776


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.217...v0.0.218

[Changes][v0.0.218]


<a name="v0.0.217"></a>
# [v0.0.217](https://github.com/astral-sh/ruff/releases/tag/v0.0.217) - 10 Jan 2023

## What's Changed
* Cache build artifacts using Swatinem/rust-cache@v1 by [@ducaale](https://github.com/ducaale) in https://github.com/charliermarsh/ruff/pull/1750
* Enable project-specific typing module re-exports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1754
* Omit `sys.version_info` and `sys.platform` checks from ternary rule by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1756
* Add a helper for any-like operations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1757
* Update rule-generation `scripts` to match latest conventions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1758
* Update documentation to match latest terminology by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1760

## New Contributors
* [@ducaale](https://github.com/ducaale) made their first contribution in https://github.com/charliermarsh/ruff/pull/1750

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.216...v0.0.217

[Changes][v0.0.217]


<a name="v0.0.216"></a>
# [v0.0.216](https://github.com/astral-sh/ruff/releases/tag/v0.0.216) - 09 Jan 2023

## What's Changed
* Audit unittest assert methods by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1736
* Rename `Check` to `Diagnostic` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1725
* Rename `CheckCategory` to `RuleOrigin` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1726
* Move violation structs out of `registry.rs` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1728
* Rename more local usages of `check` to `diagnostic` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1738
* Autofix PT004, PT005, PT024, and PT025 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1740
* Allow unused arguments for empty methods with docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1742
* Add isort.force-sort-within-sections setting by [@mattoberle](https://github.com/mattoberle) in https://github.com/charliermarsh/ruff/pull/1635
* Rename `checks` and `plugins` to `rules` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1739
* Add support for defining extra builtins by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1747
* Use dedicated warnings for flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1748


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.215...v0.0.216

[Changes][v0.0.216]


<a name="v0.0.215"></a>
# [v0.0.215](https://github.com/astral-sh/ruff/releases/tag/v0.0.215) - 08 Jan 2023

## What's Changed
* Automatically remove duplicate dictionary keys by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1710
* Add `ComparableExpr` hierarchy for comparing expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1721
* Respect isort:skip action comment by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1722
* Treat failures to fix TypedDict conversions as debug logs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1723
* Implement `--isolated` CLI flag by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1727
* Add more unittest assert methods to PT009 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1730
* Add `RUFF_FORMAT` environment variable support by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1731
* Move RUFF_CACHE_DIR to Clap's env support by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1733
* buf-fix: flake8_simplify SIM212  by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/1732
* Remove `assertNotContains` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1729


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.214...v0.0.215

[Changes][v0.0.215]


<a name="v0.0.214"></a>
# [v0.0.214](https://github.com/astral-sh/ruff/releases/tag/v0.0.214) - 07 Jan 2023

## What's Changed
* Use text in comment token by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1714
* Implement flake8-simplify SIM103 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1712
* Implement autofix for PT009 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1713
* Trim trailing whitespace when extracting isort directives by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1715
* Introduce Violation trait by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1685
* Update CONTRIBUTING.md to point to violations.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1720
* flake8_simplify : SIM210, SIM211, SIM212 by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/1717


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.213...v0.0.214

[Changes][v0.0.214]


<a name="v0.0.213"></a>
# [v0.0.213](https://github.com/astral-sh/ruff/releases/tag/v0.0.213) - 07 Jan 2023

## What's Changed
* Remove Result from SourceCodeGenerator signature by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1677
* Implement `From` conversion for style detector-to-generator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1678
* Replace `toml` with `toml_edit` by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1680
* Tweak badge logo by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1681
* Don't mark D205 as fixable in more-lines case by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1682
* Add requested context to issue template by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1679
* Update `CONTRIBUTING.md` location on `README.md` by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1688
* Implement flake8-simplify SIM108 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1684
* Remove TODO comment by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1691
* Add specialized conversions for RefEquality by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1689
* Avoiding flagging elif statements as potential ternaries by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1694
* [`flake8-bandit`] Add Rule for `S113` (requests call without timeout) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1692
* Implement flake8-simplify SIM109 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1687
* Simplify SIM201, SIM202, SIM208 by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/1666
* [`flake8-bandit`] Add Rule for `S501` (request call with `verify=False`) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1695
* Require explicit opt-in for GitHub and Gitlab formats by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1697
* Include error location in GitHub Action diagnostic messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1696
* Include list of fixed files in `stderr` output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1701
* Remove redundant #![allow()] from main_native by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1703
* Forbid unsafe code by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1704
* Switch SourceCodeGenerator.buffer from Vec<u8> to String by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1702
* Remove `add_check` methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1705
* Use `trim_end` when checking line continutation by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1706
* Automatically remove unused variables by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1683
* Lazily compute ranges for class and function bindings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1708
* Add more backticks to flake8-pytest-style error messages by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1707
* Increase blackd wait time by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1709
* Revert "Include list of fixed files in `stderr` output ([#1701](https://github.com/astral-sh/ruff/issues/1701))" by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1711


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.212...v0.0.213

[Changes][v0.0.213]


<a name="v0.0.212"></a>
# [v0.0.212](https://github.com/astral-sh/ruff/releases/tag/v0.0.212) - 06 Jan 2023

## What's Changed
* Add task-tags & ignore-overlong-task-comments settings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1654
* Stop highlighting --help output in README as shell by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1655
* Add proc-macro to derive `CheckCodePrefix` by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1656
* [`flake8-bandit`] Add Rule for `S324` (Insecure hash functions in `hashlib`) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1661
* Avoid false-positives for yields with non-identical references by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1665
* [`flake8-bandit`] Add Rule for `S506` (unsafe use of yaml load) by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1664
* Fix flake8-import-conventions configuration examples by [@diego-pm](https://github.com/diego-pm) in https://github.com/charliermarsh/ruff/pull/1660
* Allow overhang in Google-style docstring arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1668
* Fix some `&String`, `&Option`, and `&Vec` usages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1670
* Improve Pandas call and attribute detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1671
* Implement duplicate isinstance detection (SIM101) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1673
* Replace &String with &str in AnnotatedImport::ImportFrom by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1674
* Simplify Option<String> → Option<&str> conversion using as_deref by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1675
* Remove an unneeded .to_string() in tokenize_files_to_codes_mapping by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1676

## New Contributors
* [@diego-pm](https://github.com/diego-pm) made their first contribution in https://github.com/charliermarsh/ruff/pull/1660

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.211...v0.0.212

[Changes][v0.0.212]


<a name="v0.0.211"></a>
# [v0.0.211](https://github.com/astral-sh/ruff/releases/tag/v0.0.211) - 05 Jan 2023

## What's Changed
* Implement `SIM220` and `SIM221` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1630
* Implement flake8-simplify SIM105 rule by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1621
* Fix `SIM105` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1633
* Adding my company to the "used in" category of the Readme. by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1631
* Implement flake8-bandit rule `S103` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1636
* Rename flake8-bandit rules from plugins to checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1637
* Tweak Yoda condition message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1638
* Note a few more incompatibilities by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1639
* Add task-tags & ignore-overlong-task-comments settings by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1550
* Add badge JSON by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1641
* Add Ruff badge to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1642
* DRY up unused import removal code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1643
* Implement builtin import removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1645
* Move external licenses to separate directory by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1648
* Implement nested-if detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1649
* Implement flake8-bandit rule `S108` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1644
* Implement nested with detection (SIM117) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1651
* Cancel outdated in-progress workflow automatically by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1652
* Implement flake8-simplify SIM107 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1650
* Implement `SIM110` and `SIM111` (conversion to `any` and `all`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1653


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.210...v0.0.211

[Changes][v0.0.211]


<a name="v0.0.210"></a>
# [v0.0.210](https://github.com/astral-sh/ruff/releases/tag/v0.0.210) - 04 Jan 2023

## What's Changed
* Do not Change Quotation Style for `PT006` Autofix by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1600
* Implement autofix for `PT022` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1604
* Add isort.order-by-type boolean setting by [@mattoberle](https://github.com/mattoberle) in https://github.com/charliermarsh/ruff/pull/1607
* Fix *arg and **kwarg handling for Google docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1608
* Associate inline comments with parenthesized `ImportFrom` statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1609
* Fix leftover whitespace when removing `pass` for `PIE790` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1612
* Treat .pyi files as __future__ annotations-enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1616
* Treat convention as setting ignore, rather than select by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1611
* Avoid byte-string conversions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1618
* Implement missing fixes for `PT006` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1622
* Implement `yield`-to-`yield from` conversion by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1544
* Add some more users to the README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1623
* Check `SIM118` in comprehension by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1627

## New Contributors
* [@mattoberle](https://github.com/mattoberle) made their first contribution in https://github.com/charliermarsh/ruff/pull/1607

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.209...v0.0.210

[Changes][v0.0.210]


<a name="v0.0.209"></a>
# [v0.0.209](https://github.com/astral-sh/ruff/releases/tag/v0.0.209) - 03 Jan 2023

## What's Changed
* Fix several typos in README by [@jvstme](https://github.com/jvstme) in https://github.com/charliermarsh/ruff/pull/1590
* Add flake8-pytest-style settings to hash by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1595
* Add autofix for SIM300 by [@PedramNavid](https://github.com/PedramNavid) in https://github.com/charliermarsh/ruff/pull/1588
* Avoid hard unwrap in PT checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1597
* Preserve style when generating flake8-simplify messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1599
* Avoid silently dropping code generator errors by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1598
* Fix PT006 autofix of parametrize name strings like `'   first, ,  second  '` by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/1591

## New Contributors
* [@jvstme](https://github.com/jvstme) made their first contribution in https://github.com/charliermarsh/ruff/pull/1590

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.208...v0.0.209

[Changes][v0.0.209]


<a name="v0.0.208"></a>
# [v0.0.208](https://github.com/astral-sh/ruff/releases/tag/v0.0.208) - 03 Jan 2023

## What's Changed
* Adds a codespell linter by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1553
* Avoid merging import from statements with inline comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1562
* Avoid PEP 604 rewrites for runtime annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1563
* Implement `flake8-pytest-style` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1506
* Always check directly-passed-in files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1564
* Remove common-path dependency by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1565
* Rename checks.rs to registry.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1566
* Remove extend- from docstring configuration examples by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1571
* Avoid invalid trailing comma fixes for mock rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1570
* Automatically set baseline D codes based on convention by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1574
* Remove need for vendored format/cformat code by [@olliemath](https://github.com/olliemath) in https://github.com/charliermarsh/ruff/pull/1573
* Warn user when D203 and D211 are enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1576
* Add `flake8-pie` plugin with `prefer_list_builtin` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1578
* Add scripts to generate plugin and check boilerplate by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1579
* Implement unnecessary-pass-statement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1580
* Implement dupe-class-field-definitions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1581
* Implement autofix for F541 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1577
* Add a link to GitHub from the playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1583
* Mark `FStringMissingPlaceholders` as fixable by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1582
* Swap accent color for playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1584
* Prefer GitHub icon on mobile by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1585
* Implement and-false and or-true rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1586


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.207...v0.0.208

[Changes][v0.0.208]


<a name="v0.0.207"></a>
# [v0.0.207](https://github.com/astral-sh/ruff/releases/tag/v0.0.207) - 02 Jan 2023

## What's Changed
* Implement list-to-tuple comprehension unpacking by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1534
* Avoid triggering PD errors on method calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1537
* Avoid PD false positives on some non-DataFrame expressions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1538
* Correct UP027 message to “generator expression” by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1540
* Add flake8-simplify SIM300 check for Yoda Conditions by [@PedramNavid](https://github.com/PedramNavid) in https://github.com/charliermarsh/ruff/pull/1539
* Print warning when running debug builds without --no-cache by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1549
* Fix typing::match_annotated_subscript matching ExprKind::Call by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1554
* Add clarification by [@VictorGob](https://github.com/VictorGob) in https://github.com/charliermarsh/ruff/pull/1557
* Add explicit new-rule recommendation in CONTRIBUTING.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1558
* Detect unpacking assignments in eradicate by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1559
* Fix `__init__.py` being private by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1556

## New Contributors
* [@PedramNavid](https://github.com/PedramNavid) made their first contribution in https://github.com/charliermarsh/ruff/pull/1539
* [@VictorGob](https://github.com/VictorGob) made their first contribution in https://github.com/charliermarsh/ruff/pull/1557

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.206...v0.0.207

[Changes][v0.0.207]


<a name="v0.0.206"></a>
# [v0.0.206](https://github.com/astral-sh/ruff/releases/tag/v0.0.206) - 01 Jan 2023

## What's Changed
* PyUpgrade: Turn errors into OSError by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1434
* Add dark mode variant for benchmark image by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1519
* Ignore property assignments in RET504 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1520
* Avoid some false positives for ends-in-period checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1521
* Pyupgrade: `import mock` to `from unittest import mock` by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1488
* Fix `Name` node range in `NamedExpr` node by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1526
* Simplify unused snapshot check by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1525
* Do not Change Quotation Style for `SIM118` Autofix by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1529
* Add `visit_format_spec` to avoid false positives for F541 in f-string format specifier by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1528
* Rewrite mock.mock attribute accesses by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1533


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.205...v0.0.206

[Changes][v0.0.206]


<a name="v0.0.205"></a>
# [v0.0.205](https://github.com/astral-sh/ruff/releases/tag/v0.0.205) - 31 Dec 2022

## What's Changed
* Avoid flagging nested f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1516
* Use more precise error ranges for names by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1513


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.204...v0.0.205

[Changes][v0.0.205]


<a name="v0.0.204"></a>
# [v0.0.204](https://github.com/astral-sh/ruff/releases/tag/v0.0.204) - 31 Dec 2022

## What's Changed
* Trim CLI help during generation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1492
* Escape strings when formatting check messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1493
* Add a "fix message" to every autofix-able check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1489
* Stop overriding locations for expressions within f-strings by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1494
* Remove F831 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1495
* Fix detection of changed imports in isort plugin by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1504
* Remove unused snapshots by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1497
* Improve `T20X` ranges by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1502
* Improve F811 range for function and class definitions by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1499
* Improve PLW0120 range by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1500
* Fix N818 range by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1503
* Include fix commit message when showing violations together with source by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1505
* Fix E722 and F707 ranges by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1508
* Adjust `test_path` helper to detect round-trip autofix issues by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1501
* Generate source code with detected line ending by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1487
* Check for Unsupported Files and Display Errors and Warnings by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1509


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.203...v0.0.204

[Changes][v0.0.204]


<a name="v0.0.203"></a>
# [v0.0.203](https://github.com/astral-sh/ruff/releases/tag/v0.0.203) - 30 Dec 2022

## What's Changed
* Support multi-line `noqa` directives for 'import from' by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1479
* Simplified code for unicode fix by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1475
* Remove support for `ur` prefixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1481
* Detect line endings and use them during code generation by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1482
* Add a command to clear the Ruff cache by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1484
* Generate the README's --help output automatically via cargo +nightly dev generate-all by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1483
* Move some argument validation into Clap by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1485
* Remove hidden autoformat command by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1486


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.202...v0.0.203

[Changes][v0.0.203]


<a name="v0.0.202"></a>
# [v0.0.202](https://github.com/astral-sh/ruff/releases/tag/v0.0.202) - 30 Dec 2022

## What's Changed
* Make banned-api config setting optional by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1465
* Small `CONTRIBUTING.md` improvements by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1466
* Improve CLI help for `--select` by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1471
* Use more precise ranges for class and function checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1476
* Set editor background on top-level component by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1478


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.201...v0.0.202

[Changes][v0.0.202]


<a name="v0.0.201"></a>
# [v0.0.201](https://github.com/astral-sh/ruff/releases/tag/v0.0.201) - 30 Dec 2022

## What's Changed
* Rename config to settings in the playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1450
* docs(README): add missing `flake8-simplify` by [@mkniewallner](https://github.com/mkniewallner) in https://github.com/charliermarsh/ruff/pull/1449
* Add Sphinx to user list by [@AA-Turner](https://github.com/AA-Turner) in https://github.com/charliermarsh/ruff/pull/1451
* Move default options into WASM interface by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1453
* Implement dark mode by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1455
* Use trailingComma: 'all' by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1457
* Remove generated TypeScript options by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1456
* Copy URL but don't update the hash by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1458
* Removed unicode literals by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1448
* Implement TID251 (banning modules & module members) by [@not-my-profile](https://github.com/not-my-profile) in https://github.com/charliermarsh/ruff/pull/1436
* Implicit flake8-implicit-str-concat by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1463

## New Contributors
* [@mkniewallner](https://github.com/mkniewallner) made their first contribution in https://github.com/charliermarsh/ruff/pull/1449
* [@AA-Turner](https://github.com/AA-Turner) made their first contribution in https://github.com/charliermarsh/ruff/pull/1451
* [@not-my-profile](https://github.com/not-my-profile) made their first contribution in https://github.com/charliermarsh/ruff/pull/1436

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.200...v0.0.201

[Changes][v0.0.201]


<a name="v0.0.200"></a>
# [v0.0.200](https://github.com/astral-sh/ruff/releases/tag/v0.0.200) - 29 Dec 2022

## What's Changed
* Re-style the Ruff playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1438
* [pygrep-hooks] Adds Check for Blanket `# noqa` by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1440
* Avoid caching diffs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1441
* Make update check enablement cofnigurable by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1445
* Include docstrings for settings enum members by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1446


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.199...v0.0.200

[Changes][v0.0.200]


<a name="v0.0.199"></a>
# [v0.0.199](https://github.com/astral-sh/ruff/releases/tag/v0.0.199) - 29 Dec 2022

## What's Changed
* Check for keyword arguments before the last star argument by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1420
* Add Support for GitLab CI Code Quality Report Format by [@saadmk11](https://github.com/saadmk11) in https://github.com/charliermarsh/ruff/pull/1424
* Turn off wasm-pack tests by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1427
* Extract duplicated logic into method by [@hanneskaeufler](https://github.com/hanneskaeufler) in https://github.com/charliermarsh/ruff/pull/1428
* Rewrite xml.etree.cElementTree to xml.etree.ElementTree by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1426
* PyUpgrade: Replace pipes with `capture_output=True` by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1415
* Add a --diff flag to dry-run autofixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1431
* Split into lint and lint-and-fix methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1432
* Warn the user when max iteration count is reached by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1433

## New Contributors
* [@saadmk11](https://github.com/saadmk11) made their first contribution in https://github.com/charliermarsh/ruff/pull/1424

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.198...v0.0.199

[Changes][v0.0.199]


<a name="v0.0.198"></a>
# [v0.0.198](https://github.com/astral-sh/ruff/releases/tag/v0.0.198) - 28 Dec 2022

## What's Changed
* Set convention in flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1410
* Default to double quotes in `code_gen.rs` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1412
* Automatically detect and respect indentation and quotation code style by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1413
* Add rule to detect keyword arguments before starred arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1416
* Add nbQA support to the docs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1417
* Support --select ALL to enable all error codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1418


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.196...v0.0.198

[Changes][v0.0.198]


<a name="v0.0.196"></a>
# [v0.0.196](https://github.com/astral-sh/ruff/releases/tag/v0.0.196) - 27 Dec 2022

## What's Changed
* Implement pyupgrade check for io.open alias by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1399
* Tweak secret detection for playground releases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1402
* Support isort's force-single-line option by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1366
* Replace `make_tokenize` with `make_tokenizer_located` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1405
* Add cargo +nightly dev generate-all by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1404
* Allow specification of explicit docstring convention by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1408
* Pyupgrade: converts `universal_newlines` to `text` in `subprocess.run` by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1403
* Fix invalid reference to ruff_options.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1409


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.195...v0.0.196

[Changes][v0.0.196]


<a name="v0.0.195"></a>
# [v0.0.195](https://github.com/astral-sh/ruff/releases/tag/v0.0.195) - 27 Dec 2022

## What's Changed
* Add support for `ruff.toml` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1378
* Update rust python to handle files with BOM by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1379
* Only re-associate inline comments during normalization when necessary by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1380
* Magic Trailing Commas in isort by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1363
* Web playground with WASM by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1279
* Enable preview deployments for playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1383
* Add ESLint, Prettier, and TypeScript checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1384
* Add badge to playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1393
* Choose a more interesting example snippet by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1394
* Enable Quick Fix in the playground by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1395
* Only run playground release in main repo by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1396
* Now replace typing.Text with str by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1391


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.194...v0.0.195

[Changes][v0.0.195]


<a name="v0.0.194"></a>
# [v0.0.194](https://github.com/astral-sh/ruff/releases/tag/v0.0.194) - 26 Dec 2022

## What's Changed
* Fix F841 (`UnusedVariable`) range in except handler by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1367
* Improve `excepthandler_name_range` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1368
* Ignore unused arguments for [@overload](https://github.com/overload) stubs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1373
* Respect natural ordering for imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1374
* Add a `--fix-only` command-line and `pyproject.toml` option by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1375
* Avoid double-extending past the end when showing source by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1377
* Add --required-version by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1376


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.193...v0.0.194

[Changes][v0.0.194]


<a name="v0.0.193"></a>
# [v0.0.193](https://github.com/astral-sh/ruff/releases/tag/v0.0.193) - 24 Dec 2022

## What's Changed
* Update CONTRIBUTING.md by [@colin99d](https://github.com/colin99d) in https://github.com/charliermarsh/ruff/pull/1344
* Add a link to the PyCharm plugin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1345
* Avoid enabling all EM checks at once by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1349
* Implement "native literals" check from pyupgrade by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1350
* Bump compatibility to 3.11 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1352
* Add cache-dir to command-line and pyproject.toml by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1351
* Update RustPython to use the correct `BinOp` location by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1355
* Add autofix for W292 [NoNewLineAtEndOfFile] by [@Sawbez](https://github.com/Sawbez) in https://github.com/charliermarsh/ruff/pull/1354
* Don't trigger E721 when comparing with None by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1356
* Fix false-positive in RET504 when referencing globals by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1358
* Fix B025 location by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1360
* Add autofix for W605 [InvalidEscapeSequence] by [@Sawbez](https://github.com/Sawbez) in https://github.com/charliermarsh/ruff/pull/1361
* Generate JSON schema for Ruff options by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1329
* Annotate RUF100 messages with unmatched, disabled, and unknown codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1365

## New Contributors
* [@colin99d](https://github.com/colin99d) made their first contribution in https://github.com/charliermarsh/ruff/pull/1344
* [@Sawbez](https://github.com/Sawbez) made their first contribution in https://github.com/charliermarsh/ruff/pull/1354

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.192...v0.0.193

[Changes][v0.0.193]


<a name="v0.0.192"></a>
# [v0.0.192](https://github.com/astral-sh/ruff/releases/tag/v0.0.192) - 22 Dec 2022

## What's Changed
* Add some more repositories to the user list by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1328
* Allow unittest methods in flake8-boolean-trap by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1333
* Set force-exclude for pre-commit in README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1337
* Extend false-positive list for flake8-boolean-trap by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1338
* Respect --force-exclude for files passed via stdin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1342
* Implement "datetime.UTC alias" check from pyupgrade by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1341


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.191...v0.0.192

[Changes][v0.0.192]


<a name="v0.0.191"></a>
# [v0.0.191](https://github.com/astral-sh/ruff/releases/tag/v0.0.191) - 22 Dec 2022

## What's Changed
* Fix false positive DTZ001 on `datetime(2000, 1, 1, 0, 0, 0, 0, utc)` by [@bluetech](https://github.com/bluetech) in https://github.com/charliermarsh/ruff/pull/1308
* Extract line length from `pyproject.toml` Black section by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1317
* Support code redirects in flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1318
* Improve debug logging in flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1320
* Infer package roots when running via stdin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1321
* Support shell expansion in `extend` paths by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1323
* Support shell expansion in src field by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1324
* Move number of errors to the bottom of the output summary by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1325
* Implement E401 ("multiple imports on one line") by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1326

## New Contributors
* [@bluetech](https://github.com/bluetech) made their first contribution in https://github.com/charliermarsh/ruff/pull/1308

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.190...v0.0.191

[Changes][v0.0.191]


<a name="v0.0.190"></a>
# [v0.0.190](https://github.com/astral-sh/ruff/releases/tag/v0.0.190) - 21 Dec 2022

## What's Changed
* Avoid F821 false positives for Mypy extensions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1304
* Avoid flagging RUF100 as a RUF100 violation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1305
* Allow overriding cache location via RUFF_CACHE_DIR by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1312
* Avoid used-prior-global-declaration false-positives in f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1314


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.189...v0.0.190

[Changes][v0.0.190]


<a name="v0.0.189"></a>
# [v0.0.189](https://github.com/astral-sh/ruff/releases/tag/v0.0.189) - 20 Dec 2022

## What's Changed
* Update Arg section checking to match latest pydocstyle by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1293
* Avoid RET504 errors for intermediary function calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1294
* Add `--force-exclude` setting to force exclusions with `pre-commit` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1295
* [Stylistic/non-functional] Use an r# format string to make json easier to read by [@hanneskaeufler](https://github.com/hanneskaeufler) in https://github.com/charliermarsh/ruff/pull/1299
* Avoid DTZ007 false-positives for non-string arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1300

## New Contributors
* [@hanneskaeufler](https://github.com/hanneskaeufler) made their first contribution in https://github.com/charliermarsh/ruff/pull/1299

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.188...v0.0.189

[Changes][v0.0.189]


<a name="v0.0.188"></a>
# [v0.0.188](https://github.com/astral-sh/ruff/releases/tag/v0.0.188) - 19 Dec 2022

## What's Changed
* implement flake8-datetimez by [@Yasu-umi](https://github.com/Yasu-umi) in https://github.com/charliermarsh/ruff/pull/1270
* Move flake8-debugger tests into flake8-debugger subdirectory by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1286
* Avoid `T201` errors for `print(..., file=fp)`-like calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1287
* Rename PDV checks to PD by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1288

## New Contributors
* [@Yasu-umi](https://github.com/Yasu-umi) made their first contribution in https://github.com/charliermarsh/ruff/pull/1270

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.187...v0.0.188

[Changes][v0.0.188]


<a name="v0.0.187"></a>
# [v0.0.187](https://github.com/astral-sh/ruff/releases/tag/v0.0.187) - 19 Dec 2022

## What's Changed
* generate-check-code-prefix: Run `rustfmt` automatically; only write if changed by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1282
* Use `--stdin-filename` when resolving configuration files by [@cdbrendel](https://github.com/cdbrendel) in https://github.com/charliermarsh/ruff/pull/1281
* pygrep-hooks - deprecated use of logging.warn & no blanket type ignore by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1275
* Fix inverted E501 condition by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1285


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.186...v0.0.187

[Changes][v0.0.187]


<a name="v0.0.186"></a>
# [v0.0.186](https://github.com/astral-sh/ruff/releases/tag/v0.0.186) - 18 Dec 2022

## What's Changed
* Add instructions for Sublime Text installation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1271
* Add `ruff-lsp` to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1272
* Repair corrupted PDV007, PDV009 messages by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1273
* README: Add missing backtick by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1274
* Update RustPython to use correct Tuple location by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1278
* Print redirect warnings exactly once per code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1280
* Readme : Fix incorrect exmaple. by [@Honkertonken](https://github.com/Honkertonken) in https://github.com/charliermarsh/ruff/pull/1277
* Add packaging status badge from repology by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1276

## New Contributors
* [@Honkertonken](https://github.com/Honkertonken) made their first contribution in https://github.com/charliermarsh/ruff/pull/1277

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.185...v0.0.186

[Changes][v0.0.186]


<a name="v0.0.185"></a>
# [v0.0.185](https://github.com/astral-sh/ruff/releases/tag/v0.0.185) - 17 Dec 2022

## What's Changed
* Auto-detect same-package imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1266
* Separate line-based checker from `noqa` enforcement by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1267
* Move checkers into their own module by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1268


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.184...v0.0.185

[Changes][v0.0.185]


<a name="v0.0.184"></a>
# [v0.0.184](https://github.com/astral-sh/ruff/releases/tag/v0.0.184) - 16 Dec 2022

## What's Changed
* test: Fix `flake8-errmsg` snapshots by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1260
* Add ignore-variadic-names options to flake8-unused-arguments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1261
* Fix F501 (line-too-long) start location by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1262
* Replace `ignore_noqa` and `autofix` booleans with enums by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1264
* Enable autofix for __init__ method with missing None-return by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1265


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.183...v0.0.184

[Changes][v0.0.184]


<a name="v0.0.183"></a>
# [v0.0.183](https://github.com/astral-sh/ruff/releases/tag/v0.0.183) - 16 Dec 2022

## What's Changed
* Avoid fixing E711 and E712 issues that would cause F632 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1248
* Implement `pandas-vet` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1235
* Implement U016: Remove six compatibility code by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/1013
* Test to prevent continious reformatting when used together with black by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1206
* Avoid generating invalid statements when deleting from multi-statement lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1253
* Implement `flake8-errmsg` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1258
* Avoid removing partially-unused imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1259


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.182...v0.0.183

[Changes][v0.0.183]


<a name="v0.0.182"></a>
# [v0.0.182](https://github.com/astral-sh/ruff/releases/tag/v0.0.182) - 15 Dec 2022

## What's Changed
* Ignore any `pyproject.toml` without a `[tool.ruff]` section by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1243
* Treat `extend-*` configuration options as "always extended" by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1245
* Use more precise ranges for function and class checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1247


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.181...v0.0.182

[Changes][v0.0.182]


<a name="v0.0.181"></a>
# [v0.0.181](https://github.com/astral-sh/ruff/releases/tag/v0.0.181) - 14 Dec 2022

## What's Changed
* Apply fix to all errors in E711 and E712 autofix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1238
* Avoid converting expression to statement in invald contexts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1239
* Automatically ignore files specified in `.gitignore` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1234
* Add new .gitignore behavior to BREAKING_CHANGES.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1240
* Always check zero-depth CLI paths by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1241
* Enable opt-out of `.gitignore` checks via `respect-gitignore` flag by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1242


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.180...v0.0.181

[Changes][v0.0.181]


<a name="v0.0.180"></a>
# [v0.0.180](https://github.com/astral-sh/ruff/releases/tag/v0.0.180) - 14 Dec 2022

## What's Changed
* Apply CLI options even when no pyproject.toml is found by [@cdbrendel](https://github.com/cdbrendel) in https://github.com/charliermarsh/ruff/pull/1232

## New Contributors
* [@cdbrendel](https://github.com/cdbrendel) made their first contribution in https://github.com/charliermarsh/ruff/pull/1232

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.179...v0.0.180

[Changes][v0.0.180]


<a name="v0.0.179"></a>
# [v0.0.179](https://github.com/astral-sh/ruff/releases/tag/v0.0.179) - 13 Dec 2022

## What's Changed
* Upgrade RustPython to support parenthesized context managers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1228


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.178...v0.0.179

[Changes][v0.0.179]


<a name="v0.0.178"></a>
# [v0.0.178](https://github.com/astral-sh/ruff/releases/tag/v0.0.178) - 13 Dec 2022

## What's Changed
* Support hierarchical settings for nested directories by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1190
* Resolve hierarchical settings and Python files in a single filesystem pass by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1205
* Simplify some logic around configuration detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1197
* Move more commands into commands.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1209
* Use `--config` everywhere if provided by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1210
* Move Python file resolution into resolver.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1211
* Document current behavior around `pyproject.toml` discovery by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1213
* Enable configuration files to "extend" other configuration files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1219
* Add support for glob patterns in `src` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1225


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.177...v0.0.178

[Changes][v0.0.178]


<a name="v0.0.177"></a>
# [v0.0.177](https://github.com/astral-sh/ruff/releases/tag/v0.0.177) - 12 Dec 2022

## What's Changed
* Avoid inserting extra newlines for comment-delimited import blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1201
* Add notes around python-lsp-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1202
* Fix quotes in SIM118 error message by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1204


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.176...v0.0.177

[Changes][v0.0.177]


<a name="v0.0.176"></a>
# [v0.0.176](https://github.com/astral-sh/ruff/releases/tag/v0.0.176) - 11 Dec 2022

## What's Changed
* Mark C413 as fixable by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1191
* Avoid F821 false positive on annotated global by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1196
* Implement SIM118 (key in dict) of flake8-simplify by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1195
* Check for outdated auto-generated files in CI by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1192


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.175...v0.0.176

[Changes][v0.0.176]


<a name="v0.0.175"></a>
# [v0.0.175](https://github.com/astral-sh/ruff/releases/tag/v0.0.175) - 11 Dec 2022

## What's Changed
* Add jupyter_server to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1182
* Remove serialization format from Settings struct by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1183
* Add autofix for F504 and F522 by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1184
* Move string formatting checks to plugins by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1185
* Move configuration-CLI resolution into dedicated methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1188
* Enable --no-show-source for consistency by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1189
* Upgrade RustPython to fix end location of implicitly concatenated strings by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1187


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.174...v0.0.175

[Changes][v0.0.175]


<a name="v0.0.174"></a>
# [v0.0.174](https://github.com/astral-sh/ruff/releases/tag/v0.0.174) - 10 Dec 2022

## What's Changed
* Add pacman instructions to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1175
* Ignore imports in class scopes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1176
* Flag global usages prior to `global` declarations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1178
* Implement W0602 (global-variable-not-assigned) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1179
* Implement E0117 (nonlocal-without-binding) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1180
* Create function and lambda scopes eagerly by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1181


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.173...v0.0.174

[Changes][v0.0.174]


<a name="v0.0.173"></a>
# [v0.0.173](https://github.com/astral-sh/ruff/releases/tag/v0.0.173) - 10 Dec 2022

## What's Changed
* Extract docstring exactly once by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1171
* Remove some string clones from docstring helpers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1172
* Mark redefined-but-unused imports as unused regardless of scope by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1173


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.172...v0.0.173

[Changes][v0.0.173]


<a name="v0.0.172"></a>
# [v0.0.172](https://github.com/astral-sh/ruff/releases/tag/v0.0.172) - 09 Dec 2022

## What's Changed
* Clarify combination of combine-as-imports and force-wrap-aliases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1162
* Avoid RET false-positives for usages in f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1163
* Implement F842 (UnusedAnnotation) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1165
* Add pyflakes test suite for annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1166
* Always use raw docstrings for pydocstyle rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1167
* Implement D301 (backslash checks) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1169


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.171...v0.0.172

[Changes][v0.0.172]


<a name="v0.0.171"></a>
# [v0.0.171](https://github.com/astral-sh/ruff/releases/tag/v0.0.171) - 09 Dec 2022

## What's Changed
* Include else block in break detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1143
* Move bindings to an arena by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1147
* Re-implement the entire test_undefined_names.py test suite by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1150
* Implement F811 (`RedefinedWhileUnused`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1137
* Add pyflakes import test suite by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1151
* Improve some behavior around global handling by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1154
* Improve some __all__ handling cases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1155
* Run release job on release: published event by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1156
* Only allowlist noqa et al at the start of a comment by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1157


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.170...v0.0.171

[Changes][v0.0.171]


<a name="v0.0.170"></a>
# [v0.0.170](https://github.com/astral-sh/ruff/releases/tag/v0.0.170) - 08 Dec 2022

## What's Changed
* Remove 'consider' language from check messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1135
* Use single newlines in .pyi import sorting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1142


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.169...v0.0.170

[Changes][v0.0.170]


<a name="v0.0.169"></a>
# [v0.0.169](https://github.com/astral-sh/ruff/releases/tag/v0.0.169) - 08 Dec 2022

## What's Changed
* Rename I252 to TID252; add redirects for all renamed codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1129
* Don't prompt users to --fix if they ran with --fix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1133


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.168...v0.0.169

[Changes][v0.0.169]


<a name="v0.0.168"></a>
# [v0.0.168](https://github.com/astral-sh/ruff/releases/tag/v0.0.168) - 07 Dec 2022

## What's Changed
* Implement unused argument detection (`ARG`) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1126
* Convert more BTree usages to Fx by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1112


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.167...v0.0.168

[Changes][v0.0.168]


<a name="v0.0.167"></a>
# [v0.0.167](https://github.com/astral-sh/ruff/releases/tag/v0.0.167) - 07 Dec 2022

## What's Changed
* Add `flake8-import-conventions` to TOC in readme by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1114
* Add aiter() and anext() to BUILTINS by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/1118
* Reduce indents by [@youknowone](https://github.com/youknowone) in https://github.com/charliermarsh/ruff/pull/1116
* Encode prefixes in README headings not just in TOC by [@phillipuniverse](https://github.com/phillipuniverse) in https://github.com/charliermarsh/ruff/pull/1109
* Implement B905 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1122
* Auto-generate the rules table of contents by [@phillipuniverse](https://github.com/phillipuniverse) in https://github.com/charliermarsh/ruff/pull/1121
* Avoid flagging bare exception issues when exception is re-raised by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1124

## New Contributors
* [@phillipuniverse](https://github.com/phillipuniverse) made their first contribution in https://github.com/charliermarsh/ruff/pull/1109

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.166...v0.0.167

[Changes][v0.0.167]


<a name="v0.0.166"></a>
# [v0.0.166](https://github.com/astral-sh/ruff/releases/tag/v0.0.166) - 06 Dec 2022

## What's Changed
* Update readme in order to match pylint prefixes by [@billou57](https://github.com/billou57) in https://github.com/charliermarsh/ruff/pull/1105
* Avoid flagging ANN errors in [@overload](https://github.com/overload) implementations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1110
* Implement import conventions by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/1098

## New Contributors
* [@billou57](https://github.com/billou57) made their first contribution in https://github.com/charliermarsh/ruff/pull/1105

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.165...v0.0.166

[Changes][v0.0.166]


<a name="v0.0.165"></a>
# [v0.0.165](https://github.com/astral-sh/ruff/releases/tag/v0.0.165) - 06 Dec 2022

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.164...v0.0.165

[Changes][v0.0.165]


<a name="v0.0.164"></a>
# [v0.0.164](https://github.com/astral-sh/ruff/releases/tag/v0.0.164) - 06 Dec 2022

## What's Changed
* Track nested imports without column number detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1097
* Re-support F841 detection for single context managers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1099
* Auto-generate options in README from field attributes by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/1015
* Use pyproject.toml parent as project root when explicitly provided by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1101
* Improve F841's Flake8 parity for unpacking assignments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1103
* Rename rules mod to ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1104


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.163...v0.0.164

[Changes][v0.0.164]


<a name="v0.0.163"></a>
# [v0.0.163](https://github.com/astral-sh/ruff/releases/tag/v0.0.163) - 06 Dec 2022

## What's Changed
* Don't autofix D210 by introducing a syntax error by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1093
* Implement autofix for D400 and D415 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1094
* Treat nested classes and functions as "standard" siblings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1095


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.162...v0.0.163

[Changes][v0.0.163]


<a name="v0.0.162"></a>
# [v0.0.162](https://github.com/astral-sh/ruff/releases/tag/v0.0.162) - 06 Dec 2022

## What's Changed
* Ignore newline enforcement when imports break indentation boundaries by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1085
* Avoid wrapping import-star statements by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1089
* Only autofix D205 by deleting blank lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1091


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.161...v0.0.162

[Changes][v0.0.162]


<a name="v0.0.161"></a>
# [v0.0.161](https://github.com/astral-sh/ruff/releases/tag/v0.0.161) - 05 Dec 2022

## What's Changed
* Support unterminated isort: off directives by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1074
* Support isort: skip_file directive by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1075
* Import compatibility with `isort` newline-insertion behavior by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1078
* Tweak summary message to include total error counts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1067
* Support isort: split directive by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1081
* Add action comments to README.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1082


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.160...v0.0.161

[Changes][v0.0.161]


<a name="v0.0.160"></a>
# [v0.0.160](https://github.com/astral-sh/ruff/releases/tag/v0.0.160) - 05 Dec 2022

## What's Changed
* Preserve star imports when re-formatting import blocks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1066
* Include pyproject.toml path in error message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1068
* Add allowed-confusable settings by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/1059


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.159...v0.0.160

[Changes][v0.0.160]


<a name="v0.0.159"></a>
# [v0.0.159](https://github.com/astral-sh/ruff/releases/tag/v0.0.159) - 05 Dec 2022

## What's Changed
* Style fixes by [@youknowone](https://github.com/youknowone) in https://github.com/charliermarsh/ruff/pull/1049
* Upgrade to notify 5.0.0 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/1048
* Avoid false-positive on PLR1701 for multi-type isinstance calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1063
* Migrate invalid_literal_comparisons fix to token-based logic by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1065


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.158...v0.0.159

[Changes][v0.0.159]


<a name="v0.0.158"></a>
# [v0.0.158](https://github.com/astral-sh/ruff/releases/tag/v0.0.158) - 05 Dec 2022

## What's Changed
* Update RustPython by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1045
* Add an option to force one-member-per-line for aliased import-froms by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1047


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.157...v0.0.158

[Changes][v0.0.158]


<a name="v0.0.157"></a>
# [v0.0.157](https://github.com/astral-sh/ruff/releases/tag/v0.0.157) - 04 Dec 2022

## What's Changed
* Fix D205 autofix by detecting summary line by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1041
* Remove unused imports in `__init__.py` files by default by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1042


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.156...v0.0.157

[Changes][v0.0.157]


<a name="v0.0.156"></a>
# [v0.0.156](https://github.com/astral-sh/ruff/releases/tag/v0.0.156) - 04 Dec 2022

## What's Changed
* Remove sloppy match_name_or_attr helper by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/1027
* Fix README header links in isort config section by [@Jackenmen](https://github.com/Jackenmen) in https://github.com/charliermarsh/ruff/pull/1033
* Fix `PLR0402` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1024
* Fix Table of Contents by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/1030
* Implement useless-else-on-loop by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1031
* Implement `useless-import-alias` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1025
* Allow import builtins under T100 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1037
* Extend and rename RUF004 to PLR1722 by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/1036


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.155...v0.0.156

[Changes][v0.0.156]


<a name="v0.0.155"></a>
# [v0.0.155](https://github.com/astral-sh/ruff/releases/tag/v0.0.155) - 04 Dec 2022

## What's Changed
* Rename pyupgrade rules from UXXX to UPXXX by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/957
* Implement `consider-using-from-import` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1018
* Add backwards compatible redirect map for `U`-to-`UP` rename by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1019
* Rename RUF101 to RUF004 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1020
* Rename M001 to RUF100 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1021
* Implement `misplaced-comparison-constant` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1023
* Add support for combine-as-imports import formatting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1022


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.154...v0.0.155

[Changes][v0.0.155]


<a name="v0.0.154"></a>
# [v0.0.154](https://github.com/astral-sh/ruff/releases/tag/v0.0.154) - 04 Dec 2022

## What's Changed
* Implement `flake8-return` plugin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1016
* Make some `flake8-return` rules auto-fixable by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1017


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.153...v0.0.154

[Changes][v0.0.154]


<a name="v0.0.153"></a>
# [v0.0.153](https://github.com/astral-sh/ruff/releases/tag/v0.0.153) - 03 Dec 2022

## What's Changed
* Support whole-file noqa exclusions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1001
* Implement PLE0206 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1005
* Rename PLE0206 to PLR0206 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1006
* Fix `match_like_matches_macro` in `src/pylint/plugins.rs` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1007
* Implement `unnecessary-direct-lambda-call` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1008
* Fix clippy errors on main by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1010
* Implement `consider-merging-isinstance` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/1009


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.152...v0.0.153

[Changes][v0.0.153]


<a name="v0.0.152"></a>
# [v0.0.152](https://github.com/astral-sh/ruff/releases/tag/v0.0.152) - 02 Dec 2022

## What's Changed
* Add no-eval rule from pygrep-hooks by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/994
* Avoid recursing on nested deferred annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/1000


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.151...v0.0.152

[Changes][v0.0.152]


<a name="v0.0.151"></a>
# [v0.0.151](https://github.com/astral-sh/ruff/releases/tag/v0.0.151) - 02 Dec 2022

## What's Changed
* Improve docstring checks with empty trailing lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/991
* Track type definitions and annotations separately by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/992


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.150...v0.0.151

[Changes][v0.0.151]


<a name="v0.0.150"></a>
# [v0.0.150](https://github.com/astral-sh/ruff/releases/tag/v0.0.150) - 01 Dec 2022

## What's Changed
* Narrow keyword in yield-outside-function by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/971
* Implement await-outside-async / E1142 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/972
* Send logs to stderr by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/977
* Add GitHub output format by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/975
* Add Conda installation instructions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/982
* Remove trailing punctuation from error messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/983
* Convert Err(anyhow(...)) to bail by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/984
* Split test fixtures up by plugin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/985
* Remove Patch abstraction from Fix by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/987
* Include fixes in JSON API output by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/988


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.149...v0.0.150

[Changes][v0.0.150]


<a name="v0.0.149"></a>
# [v0.0.149](https://github.com/astral-sh/ruff/releases/tag/v0.0.149) - 30 Nov 2022

## What's Changed
* Add Pylint parity to FAQ by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/969
* Add JUnit xml output format by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/968


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.148...v0.0.149

[Changes][v0.0.149]


<a name="v0.0.148"></a>
# [v0.0.148](https://github.com/astral-sh/ruff/releases/tag/v0.0.148) - 30 Nov 2022

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.147...v0.0.148

[Changes][v0.0.148]


<a name="v0.0.147"></a>
# [v0.0.147](https://github.com/astral-sh/ruff/releases/tag/v0.0.147) - 30 Nov 2022

## What's Changed
* README: fixed conf section typo by [@g-as](https://github.com/g-as) in https://github.com/charliermarsh/ruff/pull/959
* Grouped format implementation by [@hay-kot](https://github.com/hay-kot) in https://github.com/charliermarsh/ruff/pull/954
* feat: use more precise ranges for imports by [@relsunkaev](https://github.com/relsunkaev) in https://github.com/charliermarsh/ruff/pull/958
* Add format setting to pyproject.toml by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/964
* Add pyupgrade's --keep-runtime-typing option by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/965
* Uses dashes for README options by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/966

## New Contributors
* [@g-as](https://github.com/g-as) made their first contribution in https://github.com/charliermarsh/ruff/pull/959
* [@hay-kot](https://github.com/hay-kot) made their first contribution in https://github.com/charliermarsh/ruff/pull/954
* [@relsunkaev](https://github.com/relsunkaev) made their first contribution in https://github.com/charliermarsh/ruff/pull/958

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.146...v0.0.147

[Changes][v0.0.147]


<a name="v0.0.146"></a>
# [v0.0.146](https://github.com/astral-sh/ruff/releases/tag/v0.0.146) - 29 Nov 2022

## What's Changed
* Allow preservation of external check codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/955
* Remove pre-commit note in README.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/956


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.145...v0.0.146

[Changes][v0.0.146]


<a name="v0.0.145"></a>
# [v0.0.145](https://github.com/astral-sh/ruff/releases/tag/v0.0.145) - 29 Nov 2022

## What's Changed
* Implement eradicate by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/947
* Rewrite type annotations on Python 3.7 when __future__ enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/953


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.144...v0.0.145

[Changes][v0.0.145]


<a name="v0.0.144"></a>
# [v0.0.144](https://github.com/astral-sh/ruff/releases/tag/v0.0.144) - 29 Nov 2022

## What's Changed
* fix(flake8_boolean_trap): add whitelist for dict methods by [@pwoolvett](https://github.com/pwoolvett) in https://github.com/charliermarsh/ruff/pull/943
* Allow long lines that consist of only a URL by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/952


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.143...v0.0.144

[Changes][v0.0.144]


<a name="v0.0.143"></a>
# [v0.0.143](https://github.com/astral-sh/ruff/releases/tag/v0.0.143) - 28 Nov 2022

## What's Changed
* Don’t require files with --explain or --generate-shell-completion by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/937
* Allow `@override` methods to be undocumented by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/941
* Fix clippy::manual_let_else (pedantic) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/939


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.142...v0.0.143

[Changes][v0.0.143]


<a name="v0.0.142"></a>
# [v0.0.142](https://github.com/astral-sh/ruff/releases/tag/v0.0.142) - 28 Nov 2022

## What's Changed
* Track aliased import-from members by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/929
* F50x implementation by [@olliemath](https://github.com/olliemath) in https://github.com/charliermarsh/ruff/pull/919
* Add Homebrew installation to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/930
* Use alternative TOML format for per-file-ignores in README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/931
* Add some user testimonials by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/932
* Allow varargs and kwargs to be prefixed with stars by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/933
* Do not enforce line length limit for comments ending with a URL by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/920
* Document all top-level configuration options by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/934
* Add shell completions support by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/935
* Add all plugin options to README reference by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/936


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.141...v0.0.142

[Changes][v0.0.142]


<a name="v0.0.141"></a>
# [v0.0.141](https://github.com/astral-sh/ruff/releases/tag/v0.0.141) - 26 Nov 2022

## What's Changed
* Add flake8-debugger by [@karpa4o4](https://github.com/karpa4o4) in https://github.com/charliermarsh/ruff/pull/909
* Avoid flagging redundant open modes when open is rebound by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/918

## New Contributors
* [@karpa4o4](https://github.com/karpa4o4) made their first contribution in https://github.com/charliermarsh/ruff/pull/909

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.140...v0.0.141

[Changes][v0.0.141]


<a name="v0.0.140"></a>
# [v0.0.140](https://github.com/astral-sh/ruff/releases/tag/v0.0.140) - 26 Nov 2022

## What's Changed
* Fix F821 false positive by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/911
* Respect f-string locations in B023 check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/914
* Respect noqa comments in U009 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/917
* Auto-generate CheckCodePrefix::fixables() by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/916
* Preserve existing noqa codes in --add-noqa by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/913


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.139...v0.0.140

[Changes][v0.0.140]


<a name="v0.0.139"></a>
# [v0.0.139](https://github.com/astral-sh/ruff/releases/tag/v0.0.139) - 25 Nov 2022

## What's Changed
* Minor changes in README. by [@huxuan](https://github.com/huxuan) in https://github.com/charliermarsh/ruff/pull/903
* Implement F522-F525 by [@olliemath](https://github.com/olliemath) in https://github.com/charliermarsh/ruff/pull/899
* Implement B023 (function uses loop variable) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/907
* Add keyword argument handling for redundant open modes. by [@andribergs](https://github.com/andribergs) in https://github.com/charliermarsh/ruff/pull/906

## New Contributors
* [@huxuan](https://github.com/huxuan) made their first contribution in https://github.com/charliermarsh/ruff/pull/903

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.138...v0.0.139

[Changes][v0.0.139]


<a name="v0.0.138"></a>
# [v0.0.138](https://github.com/astral-sh/ruff/releases/tag/v0.0.138) - 25 Nov 2022

## What's Changed
* Implement B904 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/892
* Implement F521 by [@olliemath](https://github.com/olliemath) in https://github.com/charliermarsh/ruff/pull/898
* Issue 662 explore globset by [@CelebrateVC](https://github.com/CelebrateVC) in https://github.com/charliermarsh/ruff/pull/883
* Remove UserConfiguration struct by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/900
* Move some main.rs subcommands to a new module by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/901
* Fix typo by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/902

## New Contributors
* [@olliemath](https://github.com/olliemath) made their first contribution in https://github.com/charliermarsh/ruff/pull/898
* [@CelebrateVC](https://github.com/CelebrateVC) made their first contribution in https://github.com/charliermarsh/ruff/pull/883

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.137...v0.0.138

[Changes][v0.0.138]


<a name="v0.0.137"></a>
# [v0.0.137](https://github.com/astral-sh/ruff/releases/tag/v0.0.137) - 24 Nov 2022

## What's Changed
* Treat withitem variables as bindings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/897


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.136...v0.0.137

[Changes][v0.0.137]


<a name="v0.0.136"></a>
# [v0.0.136](https://github.com/astral-sh/ruff/releases/tag/v0.0.136) - 23 Nov 2022

## What's Changed
* Set rust-version in Cargo.toml by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/886
* Add `--explain` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/887
* Upload wheels back to GitHub Releases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/884
* Visit iter prior to target in comprehensions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/895


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.135...v0.0.136

[Changes][v0.0.136]


<a name="v0.0.135"></a>
# [v0.0.135](https://github.com/astral-sh/ruff/releases/tag/v0.0.135) - 23 Nov 2022

## What's Changed
* Fix clippy::default-trait-access (pedantic) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/867
* Fix most clippy::pedantic lints by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/869
* Update README with list of projects by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/874
* Apply autofixes iteratively until code is stabilized by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/875
* Return `Vec<Check>` from check_tokens by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/876
* Remove Mode from various internal checkers by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/877
* Remove always-inline by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/879
* Fix clippy::unnecessary_wraps (pedantic) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/880
* Enforce most pedantic lints on CI by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/878
* Apply a limit to the number of fix iterations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/882
* Log errors in add_noqa and autoformat calls by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/881


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.134...v0.0.135

[Changes][v0.0.135]


<a name="v0.0.134"></a>
# [v0.0.134](https://github.com/astral-sh/ruff/releases/tag/v0.0.134) - 21 Nov 2022

## What's Changed
* Fix clippy::inefficient-to-string (pedantic) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/860
* Fix clippy::sort-unstable (pedantic) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/861
* Fix clippy::trivially-copy-pass-by-ref (pedantic) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/862
* Ignore clippy::match-same-arms (pedantic) in a few places by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/863
* Ignore clippy::unreadable-literal (pedantic) for CONFUSABLES by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/864
* Automatically remove redundant open modes [#640](https://github.com/astral-sh/ruff/issues/640) by [@andberger](https://github.com/andberger) in https://github.com/charliermarsh/ruff/pull/843

## New Contributors
* [@andberger](https://github.com/andberger) made their first contribution in https://github.com/charliermarsh/ruff/pull/843

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.133...v0.0.134

[Changes][v0.0.134]


<a name="v0.0.133"></a>
# [v0.0.133](https://github.com/astral-sh/ruff/releases/tag/v0.0.133) - 21 Nov 2022

## What's Changed
* Implement autofix for B013 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/824
* Avoid attempting to fix PEP 604 violations with deferred annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/845
* Upgrade maturin to 0.14 by [@messense](https://github.com/messense) in https://github.com/charliermarsh/ruff/pull/846
* Make it visible under light theme by [@kemingy](https://github.com/kemingy) in https://github.com/charliermarsh/ruff/pull/854
* Upgrade RustPython by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/855
* Sort relative imports by parent level descending by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/856
* Avoid incrementing McCabe complexity for class methods by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/857
* Add unit tests for complexity check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/859
* Propagate errors from glob::Pattern::new by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/858

## New Contributors
* [@messense](https://github.com/messense) made their first contribution in https://github.com/charliermarsh/ruff/pull/846
* [@kemingy](https://github.com/kemingy) made their first contribution in https://github.com/charliermarsh/ruff/pull/854

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.132...v0.0.133

[Changes][v0.0.133]


<a name="v0.0.132"></a>
# [v0.0.132](https://github.com/astral-sh/ruff/releases/tag/v0.0.132) - 20 Nov 2022

## What's Changed
* Replace FNV with rustc-hash by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/837
* Add RUF to list of fixable defaults by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/838
* Fix N804 class method with positional only args by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/836
* Support PEP 562 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/841
* Add convert exit() to sys.exit() rule by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/816


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.131...v0.0.132

[Changes][v0.0.132]


<a name="v0.0.131"></a>
# [v0.0.131](https://github.com/astral-sh/ruff/releases/tag/v0.0.131) - 20 Nov 2022

## What's Changed
* Add CACHEDIR.TAG to .ruff_cache by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/830
* Make main.rs robust to cache initialization failures by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/831
* Improve cache performance by removing `cacache` dependency by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/833


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.130...v0.0.131

[Changes][v0.0.131]


<a name="v0.0.130"></a>
# [v0.0.130](https://github.com/astral-sh/ruff/releases/tag/v0.0.130) - 20 Nov 2022

## What's Changed
* Adjust U011 start location by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/828
* Implement autofix for B010 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/823
* Implement U014: Convert NamedTuple function to class by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/819
* Add class names to NamedTuple and TypedDict rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/829


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.129...v0.0.130

[Changes][v0.0.130]


<a name="v0.0.129"></a>
# [v0.0.129](https://github.com/astral-sh/ruff/releases/tag/v0.0.129) - 20 Nov 2022

## What's Changed
* U013: Also convert typing.TypedDict by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/810
* Adjust `UnusedNOQA` start location by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/817
* Mark nonlocal variables as used in parent scopes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/822
* Exempt parameters with immutable annotations from B006 by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/821
* Implement autofix for E731 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/814


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.128...v0.0.129

[Changes][v0.0.129]


<a name="v0.0.128"></a>
# [v0.0.128](https://github.com/astral-sh/ruff/releases/tag/v0.0.128) - 18 Nov 2022

## What's Changed
* Implement a `--show-source` setting by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/698
* Remove warn_on checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/812
* Enable customization of autofixable error codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/811


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.127...v0.0.128

[Changes][v0.0.128]


<a name="v0.0.127"></a>
# [v0.0.127](https://github.com/astral-sh/ruff/releases/tag/v0.0.127) - 18 Nov 2022

## What's Changed
* Implement C901 (mccabe) by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/765
* Add missing plugins in some sections of README.md by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/802
* Implement U013: Unnecessary TypedDict syntactic form by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/716
* Misc. follow-ups to [#716](https://github.com/astral-sh/ruff/issues/716) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/806
* Add flake8-blind-except by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/805
* Implement autofix for E713 and E714 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/804
* Reduce newlines in code gen by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/807
* Add flake8-boolean-trap by [@pwoolvett](https://github.com/pwoolvett) in https://github.com/charliermarsh/ruff/pull/790
* Change error code of flake8-blind-except by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/808

## New Contributors
* [@pwoolvett](https://github.com/pwoolvett) made their first contribution in https://github.com/charliermarsh/ruff/pull/790

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.126...v0.0.127

[Changes][v0.0.127]


<a name="v0.0.126"></a>
# [v0.0.126](https://github.com/astral-sh/ruff/releases/tag/v0.0.126) - 17 Nov 2022

## What's Changed
* Trim dedented sections for arg detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/793
* Trigger N818 when parent ends in Error or Exception by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/796
* Fix D202 to remove line after docstring by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/797
* Except BaseException from N818 checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/798
* Ignore globals when checking local variable names by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/800


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.125...v0.0.126

[Changes][v0.0.126]


<a name="v0.0.125"></a>
# [v0.0.125](https://github.com/astral-sh/ruff/releases/tag/v0.0.125) - 17 Nov 2022

## What's Changed
* Fix find_and_parse_pyproject_toml test for [#772](https://github.com/astral-sh/ruff/issues/772) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/774
* Propagate exit code through Python __main__ wrapper by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/776
* Remove unnecessary abspath rule (U002) by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/781
* Add the tools identifier in the TOC by [@JonathanPlasse](https://github.com/JonathanPlasse) in https://github.com/charliermarsh/ruff/pull/779
* Implement auto-fix for E711 and E712 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/784
* Implement flake8-tidy-imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/789
* docs(integrations): neovim `null-ls` integration by [@eddiebergman](https://github.com/eddiebergman) in https://github.com/charliermarsh/ruff/pull/782
* Tweak presentation of `null-ls` and `efm` docs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/791

## New Contributors
* [@JonathanPlasse](https://github.com/JonathanPlasse) made their first contribution in https://github.com/charliermarsh/ruff/pull/781
* [@eddiebergman](https://github.com/eddiebergman) made their first contribution in https://github.com/charliermarsh/ruff/pull/782

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.124...v0.0.125

[Changes][v0.0.125]


<a name="v0.0.124"></a>
# [v0.0.124](https://github.com/astral-sh/ruff/releases/tag/v0.0.124) - 16 Nov 2022

## What's Changed
* Support arbitrary expression paths for class and static decorators by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/772


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.123...v0.0.124

[Changes][v0.0.124]


<a name="v0.0.123"></a>
# [v0.0.123](https://github.com/astral-sh/ruff/releases/tag/v0.0.123) - 16 Nov 2022

## What's Changed
* Avoid allocations for binding values by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/764
* Fix E731 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/766
* Change all &Option<> to Option<&> by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/768
* Fix off-by-one in noqa map detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/771


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.122...v0.0.123

[Changes][v0.0.123]


<a name="v0.0.122"></a>
# [v0.0.122](https://github.com/astral-sh/ruff/releases/tag/v0.0.122) - 16 Nov 2022

## What's Changed
* Increase retry counts in GitHub Actions workflows by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/763
* Preserve comments when sorting imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/749


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.121...v0.0.122

[Changes][v0.0.122]


<a name="v0.0.121"></a>
# [v0.0.121](https://github.com/astral-sh/ruff/releases/tag/v0.0.121) - 15 Nov 2022

## What's Changed
* docs: Add `flake8-bandit` to ToC by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/750
* Limit PEP 604 checks to Python 3.10+ by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/757
* Preserve scopes when checking deferred strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/758
* Add --line-length command line argument by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/759
* Disable auto-updates in JSON mode by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/760
* Only print version checks on tty by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/761
* Only notify once for each app update by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/762
* Implement B020 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/753


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.120...v0.0.121

[Changes][v0.0.121]


<a name="v0.0.120"></a>
# [v0.0.120](https://github.com/astral-sh/ruff/releases/tag/v0.0.120) - 15 Nov 2022

## What's Changed
* De-alias Literal checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/748


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.119...v0.0.120

[Changes][v0.0.120]


<a name="v0.0.119"></a>
# [v0.0.119](https://github.com/astral-sh/ruff/releases/tag/v0.0.119) - 15 Nov 2022

## What's Changed
* Improve performance of import matching code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/744
* Add isort to the README's ToC by [@brettcannon](https://github.com/brettcannon) in https://github.com/charliermarsh/ruff/pull/745
* Add support for import alias tracking by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/746
* Move bindings to FNV map by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/747


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.118...v0.0.119

[Changes][v0.0.119]


<a name="v0.0.118"></a>
# [v0.0.118](https://github.com/astral-sh/ruff/releases/tag/v0.0.118) - 14 Nov 2022

## What's Changed
* Allow explicit re-export of straight imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/729
* Add FastAPI to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/730
* Make `combine-as-imports` the default import sorting behavior by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/731
* Use FNV hasher in more places by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/732
* Implement B022 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/734
* Add flake8-bugbear settings to hash by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/739
* Allow second line as 'first line' for punctuation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/741
* Implement B024 and B027 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/738
* Ignore namedtuple assignment in N806, N815, and N816 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/735


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.117...v0.0.118

[Changes][v0.0.118]


<a name="v0.0.117"></a>
# [v0.0.117](https://github.com/astral-sh/ruff/releases/tag/v0.0.117) - 13 Nov 2022

## What's Changed
* Don't mark re-exported symbols as unused by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/724
* Restore clippy on all crates in the workspace by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/725
* Fix Markdown in README by [@brettcannon](https://github.com/brettcannon) in https://github.com/charliermarsh/ruff/pull/727
* Make # noqa detection case-insensitive by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/728

## New Contributors
* [@brettcannon](https://github.com/brettcannon) made their first contribution in https://github.com/charliermarsh/ruff/pull/727

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.116...v0.0.117

[Changes][v0.0.117]


<a name="v0.0.116"></a>
# [v0.0.116](https://github.com/astral-sh/ruff/releases/tag/v0.0.116) - 13 Nov 2022

## What's Changed
* Improve some import tracking code by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/715
* Implement B021 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/719
* Implement B012 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/718
* Lint test code by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/721


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.115...v0.0.116

[Changes][v0.0.116]


<a name="v0.0.115"></a>
# [v0.0.115](https://github.com/astral-sh/ruff/releases/tag/v0.0.115) - 12 Nov 2022

## What's Changed
* Add `flake8-bandit` by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/697
* Add flake8-bandit to flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/701
* Add `extend-immutable-calls` setting for B008 by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/706
* Use FnvHasher for unordered maps and sets by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/708
* Track all import-from members by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/709
* Validate that mutable and immutable defaults are imported by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/710
* Use an FNVHashSet for `settings.enabled` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/711
* Include flake8-bugbear settings in flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/712


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.114...v0.0.115

[Changes][v0.0.115]


<a name="v0.0.114"></a>
# [v0.0.114](https://github.com/astral-sh/ruff/releases/tag/v0.0.114) - 12 Nov 2022

## What's Changed
* Take indentation into account for import-from wrapping by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/693
* Disable default features of chrono by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/696
* Remove static isort classifications for __main__, disutils by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/694
* Implement B019 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/695
* add fixes for __future__ import removal by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/682
* Avoid generating empty statement bodies by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/700
* feat: no unnecessary encode utf8 by [@martinlehoux](https://github.com/martinlehoux) in https://github.com/charliermarsh/ruff/pull/686

## New Contributors
* [@martinlehoux](https://github.com/martinlehoux) made their first contribution in https://github.com/charliermarsh/ruff/pull/686

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.113...v0.0.114

[Changes][v0.0.114]


<a name="v0.0.113"></a>
# [v0.0.113](https://github.com/astral-sh/ruff/releases/tag/v0.0.113) - 12 Nov 2022

## What's Changed
* Implement flake8-2020 (sys.version, sys.version_info misuse) by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/688
* Add a separate local folder category for imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/690
* Default to isort's import sort logic by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/691


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.112...v0.0.113

[Changes][v0.0.113]


<a name="v0.0.112"></a>
# [v0.0.112](https://github.com/astral-sh/ruff/releases/tag/v0.0.112) - 11 Nov 2022

## What's Changed
* Only scan checks once in check_lines by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/679
* Add ruff.__main__ wrapper to allow invocation as ‘python -m ruff’ by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/687


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.111...v0.0.112

[Changes][v0.0.112]


<a name="v0.0.111"></a>
# [v0.0.111](https://github.com/astral-sh/ruff/releases/tag/v0.0.111) - 11 Nov 2022

## What's Changed
* Clarify a few settings for isort behavior by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/676
* Implement B010 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/683
* Implement autofix for B009 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/684
* Fix lambda handling for B010 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/685
* Support `isort: skip`, `isort: on`, and `isort: off` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/678


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.110...v0.0.111

[Changes][v0.0.111]


<a name="v0.0.110"></a>
# [v0.0.110](https://github.com/astral-sh/ruff/releases/tag/v0.0.110) - 11 Nov 2022

## What's Changed
* Implement B009 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/669
* Limit Ropey to newlines and carriage returns by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/670
* Rename some fixture files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/671
* Implement import sorting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/633
* Add a test utility for running lint checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/672


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.109...v0.0.110

[Changes][v0.0.110]


<a name="v0.0.109"></a>
# [v0.0.109](https://github.com/astral-sh/ruff/releases/tag/v0.0.109) - 10 Nov 2022

## What's Changed
* Autofix C413 by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/661
* Add notes to README on editor integrations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/655
* Upgrade LibCST and other crates by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/663
* Detect unnecessary params in `lru_cache`  by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/664
* Fix unnecessary params in `lru_cache` by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/667
* Implement B026 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/668


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.108...v0.0.109

[Changes][v0.0.109]


<a name="v0.0.108"></a>
# [v0.0.108](https://github.com/astral-sh/ruff/releases/tag/v0.0.108) - 08 Nov 2022

## What's Changed
* Upgrade RustPython by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/652
* Implement confusing unicode character detection for comments by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/653
* remove unnecessary __future__ imports by [@chammika-become](https://github.com/chammika-become) in https://github.com/charliermarsh/ruff/pull/634
* Implement fix for C404 by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/656
* Implement ANN401 by [@edgarrmondragon](https://github.com/edgarrmondragon) in https://github.com/charliermarsh/ruff/pull/657

## New Contributors
* [@chammika-become](https://github.com/chammika-become) made their first contribution in https://github.com/charliermarsh/ruff/pull/634
* [@edgarrmondragon](https://github.com/edgarrmondragon) made their first contribution in https://github.com/charliermarsh/ruff/pull/657

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.107...v0.0.108

[Changes][v0.0.108]


<a name="v0.0.107"></a>
# [v0.0.107](https://github.com/astral-sh/ruff/releases/tag/v0.0.107) - 07 Nov 2022

## What's Changed
* Avoid U009 violations when disabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/650
* Run annotations plugin if ANN204, ANN205, ANN206 are selected by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/649


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.106...v0.0.107

[Changes][v0.0.107]


<a name="v0.0.106"></a>
# [v0.0.106](https://github.com/astral-sh/ruff/releases/tag/v0.0.106) - 07 Nov 2022

## What's Changed
* Infer plugins based on per-file-ignores, ignores, etc. by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/632
* Fix --ignore for ANN101 and ANN102 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/637
* Implement B004 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/638
* Add fix option to `pyproject.toml` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/639
* Implement B005 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/643
* Add a flake8-to-ruff mention by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/644
* Include function and argument names in ANN checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/648


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.105...v0.0.106

[Changes][v0.0.106]


<a name="v0.0.105"></a>
# [v0.0.105](https://github.com/astral-sh/ruff/releases/tag/v0.0.105) - 07 Nov 2022

## What's Changed
* Implement flake8-annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/625
* Remove CheckLocator abstraction by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/627
* Remove erroneous Literal entry from subscript list by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/630
* Respect project root in per-file ignores by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/631


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.104...v0.0.105

[Changes][v0.0.105]


<a name="v0.0.104"></a>
# [v0.0.104](https://github.com/astral-sh/ruff/releases/tag/v0.0.104) - 06 Nov 2022

## What's Changed
* Fix B015 false positive on comparison deep inside expression statement by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/616
* Correct source link in CONFUSABLES comment by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/617
* pyflakes F632 Autofix by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/612
* Allow underscore names in N803 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/622
* Remove utf-8 encoding declaration by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/618
* Improve discoverability of dev commands by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/621
* Update CONTRIBUTING.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/623
* Categorize functions in pep8-naming by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/624


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.103...v0.0.104

[Changes][v0.0.104]


<a name="v0.0.103"></a>
# [v0.0.103](https://github.com/astral-sh/ruff/releases/tag/v0.0.103) - 05 Nov 2022

## What's Changed
* Automatically write to src/checks_gen.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/604
* Remove rust version from CONTRIBUTING.md by [@squiddy](https://github.com/squiddy) in https://github.com/charliermarsh/ruff/pull/605
* Automatically update README.md from generate_rules_table.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/606
* Create a separate dev crate for development scripts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/607
* Add a list of projects using Ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/608
* Remove some usages of Ruff internals in ruff_dev by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/610
* Add a README link to each plugin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/611
* Fix “Code” misspelling by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/614
* Only track noqa directives for multi-line strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/615

## New Contributors
* [@squiddy](https://github.com/squiddy) made their first contribution in https://github.com/charliermarsh/ruff/pull/605

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.102...v0.0.103

[Changes][v0.0.103]


<a name="v0.0.102"></a>
# [v0.0.102](https://github.com/astral-sh/ruff/releases/tag/v0.0.102) - 05 Nov 2022

## What's Changed
* Implement B015 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/587
* Fix invalid escape handling for CRLF files by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/589
* Split ambiguous unicode detection into string vs. docstring rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/590
* Change Ruff's error prefix to RUF by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/592
* Rely on token locations for noqa map extraction by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/603
* Implement B008 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/594
* Implement B016 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/595
* Implement B003 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/596
* Remove needless return by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/597
* Ignore ellipsis in B018 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/598


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.100...v0.0.102

[Changes][v0.0.102]


<a name="v0.0.100"></a>
# [v0.0.100](https://github.com/astral-sh/ruff/releases/tag/v0.0.100) - 04 Nov 2022

## What's Changed
* Implement autofix for F901 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/571
* Infer Flake8 plugins from .flake8 config by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/573
* Add W to list of default flake8-to-ruff codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/574
* Bump Rust version to 1.65.0 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/575
* Use a rope to manage string slicing by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/576
* Confine subscript annotation checks to `ExprContext::Load` by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/583
* Implement B018 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/582
* Use a Rope to power fixer by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/584
* Use a shared Rope between AST checker and fixer by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/585
* Implement ambiguous unicode character detection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/578


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.99...v0.0.100

[Changes][v0.0.100]


<a name="v0.0.99"></a>
# [v0.0.99](https://github.com/astral-sh/ruff/releases/tag/v0.0.99) - 03 Nov 2022

## What's Changed
* Enable autofix for dict(a=1)-like dictionaries by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/567
* Implement autofix for C416 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/568
* Enable autofix for C406 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/570


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.98...v0.0.99

[Changes][v0.0.99]


<a name="v0.0.98"></a>
# [v0.0.98](https://github.com/astral-sh/ruff/releases/tag/v0.0.98) - 03 Nov 2022

## What's Changed
* Remove crates subdirectory by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/563
* Make --quiet more aggressive by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/566


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.97...v0.0.98

[Changes][v0.0.98]


<a name="v0.0.97"></a>
# [v0.0.97](https://github.com/astral-sh/ruff/releases/tag/v0.0.97) - 03 Nov 2022

## What's Changed
* Avoid autofixes for errors in f-strings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/561
* Relax lowercase condition in N806 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/562


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.96...v0.0.97

[Changes][v0.0.97]


<a name="v0.0.96"></a>
# [v0.0.96](https://github.com/astral-sh/ruff/releases/tag/v0.0.96) - 03 Nov 2022

## What's Changed
* Set override in actions-rs/toolchain@v1 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/543
* Expose autofix mode in public API by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/551
* Automatically fix a variety of comprehension rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/553
* Respect trailing whitespace in comprehension fixes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/554
* Implement autofix for dict and tuple comprehensions by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/555
* DRY up utilities in flake8_comprehensions/fixes.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/556
* Change flake8-quotes default to double quotes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/558
* Add plugin properties to settings cache key by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/559


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.95...v0.0.96

[Changes][v0.0.96]


<a name="v0.0.95"></a>
# [v0.0.95](https://github.com/astral-sh/ruff/releases/tag/v0.0.95) - 02 Nov 2022

## What's Changed
* Add plugin configuration to flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/535
* Make columns indices 1-based in the text output format by [@fsouza](https://github.com/fsouza) in https://github.com/charliermarsh/ruff/pull/539
* Use nightly rustfmt with rustfmt.toml by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/536
* Add a rust-toolchain.toml file by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/538
* Use max-line-length in converter.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/541
* Add tests for converter.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/542
* Account for typing_extensions for annotation parsing by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/550
* Update README.md to use table for per-file-ignore by [@StefanBRas](https://github.com/StefanBRas) in https://github.com/charliermarsh/ruff/pull/549

## New Contributors
* [@StefanBRas](https://github.com/StefanBRas) made their first contribution in https://github.com/charliermarsh/ruff/pull/549

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.94...v0.0.95

[Changes][v0.0.95]


<a name="v0.0.94"></a>
# [v0.0.94](https://github.com/astral-sh/ruff/releases/tag/v0.0.94) - 01 Nov 2022

## What's Changed
* Add a Flake8-to-Ruff configuration conversion tool by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/527
* Move flake8-to-ruff to a separate crate by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/528
* Add a separate release job for flake8-to-ruff by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/529
* Use more consistent Option in pyproject settings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/530
* Represent per-file ignores as a map by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/531
* Track typing module imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/533
* Refine list of annotatable subscripts by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/534


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.93...v0.0.94

[Changes][v0.0.94]


<a name="v0.0.93"></a>
# [v0.0.93](https://github.com/astral-sh/ruff/releases/tag/v0.0.93) - 31 Oct 2022

## What's Changed
* Modify public API to return Check rather than Message by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/524


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.92...v0.0.93

[Changes][v0.0.93]


<a name="v0.0.92"></a>
# [v0.0.92](https://github.com/astral-sh/ruff/releases/tag/v0.0.92) - 30 Oct 2022

## What's Changed
* Avoid re-indenting empty lines in D207 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/517
* Implement B006 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/515
* Avoid flagging D202 for inner functions and classes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/518
* Implement consistent newline handling for SourceCodeLocator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/519
* Tweak a few check messages by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/520
* Add a cargo bench for SourceCodeLocator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/521
* Move SourceCodeLocator to its own module by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/522
* Remove RustPython fork by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/523


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.91...v0.0.92

[Changes][v0.0.92]


<a name="v0.0.91"></a>
# [v0.0.91](https://github.com/astral-sh/ruff/releases/tag/v0.0.91) - 29 Oct 2022

## What's Changed
* Avoid flake8-comprehensions errors for dicts with kwargs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/512


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.90...v0.0.91

[Changes][v0.0.91]


<a name="v0.0.90"></a>
# [v0.0.90](https://github.com/astral-sh/ruff/releases/tag/v0.0.90) - 29 Oct 2022

## What's Changed
* Add error code categories to table of contents by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/504
* Implement configuration options for pep8-naming by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/505
* Move pyproject.toml logging to debug by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/506
* Remove leading space from C416 message by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/508
* Simplify SourceCodeLocator offset computation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/509
* Use a single SourceCodeLocator everywhere by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/510


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.89...v0.0.90

[Changes][v0.0.90]


<a name="v0.0.89"></a>
# [v0.0.89](https://github.com/astral-sh/ruff/releases/tag/v0.0.89) - 29 Oct 2022

## What's Changed
* Implement N806, 815, 816, 818 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/501
* Ignore unittest methods and functions in N802 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/502
* Implement B013 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/503


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.88...v0.0.89

[Changes][v0.0.89]


<a name="v0.0.88"></a>
# [v0.0.88](https://github.com/astral-sh/ruff/releases/tag/v0.0.88) - 28 Oct 2022

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.87...v0.0.88

[Changes][v0.0.88]


<a name="v0.0.87"></a>
# [v0.0.87](https://github.com/astral-sh/ruff/releases/tag/v0.0.87) - 28 Oct 2022

## What's Changed
* Update hook id in README and in .pre-commit-config.yaml by [@tgross35](https://github.com/tgross35) in https://github.com/charliermarsh/ruff/pull/492
* Move invalid_escape_sequence into pycodestyle by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/494
* Implement the `flake8-quotes` plugin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/495
* Enable prefix-based check code selection by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/493
* Move around and rename some of the Settings structs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/496
* Add tests for resolve_codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/498
* Fix “not a char boundary” error with Unicode in extract_quote by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/497

## New Contributors
* [@tgross35](https://github.com/tgross35) made their first contribution in https://github.com/charliermarsh/ruff/pull/492

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.86...v0.0.87

[Changes][v0.0.87]


<a name="v0.0.86"></a>
# [v0.0.86](https://github.com/astral-sh/ruff/releases/tag/v0.0.86) - 27 Oct 2022

## What's Changed
* Replace compliance comments with check codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/485
* Allow whitespace in per-file ignore patterns by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/487
* Add example of per-file ignores to the README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/488
* Avoid auto-fixing unused imports in __init__.py by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/489


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.85...v0.0.86

[Changes][v0.0.86]


<a name="v0.0.85"></a>
# [v0.0.85](https://github.com/astral-sh/ruff/releases/tag/v0.0.85) - 26 Oct 2022

## What's Changed
* Rename --quiet to --silent and make --quiet only log errors by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/477
* Suppress “No pyproject.toml found” message with --quiet by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/478
* Handle multi-segment import-from removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/479
* Fix multi-segment import removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/480
* Implement W605 (invalid escape sequence) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/482


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.84...v0.0.85

[Changes][v0.0.85]


<a name="v0.0.84"></a>
# [v0.0.84](https://github.com/astral-sh/ruff/releases/tag/v0.0.84) - 26 Oct 2022

## What's Changed
* DRY up usages of matches with fixer Mode by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/470
* Correct EOL offset for lines ending with multi-byte char by [@sgryjp](https://github.com/sgryjp) in https://github.com/charliermarsh/ruff/pull/471
* Use lazy initialization for SourceCodeLocator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/472
* Implement B007 (unused loop control variable) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/473


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.83...v0.0.84

[Changes][v0.0.84]


<a name="v0.0.83"></a>
# [v0.0.83](https://github.com/astral-sh/ruff/releases/tag/v0.0.83) - 26 Oct 2022

## What's Changed
* Enable N811, 812, 813, 814, 817 for `Import` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/460
* Fix uppercase and lowercase check by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/461
* Restyle flake8_comprehensions::check to reduce indent by [@youknowone](https://github.com/youknowone) in https://github.com/charliermarsh/ruff/pull/462
* chore: typo on [#283](https://github.com/astral-sh/ruff/issues/283) link by [@fannheyward](https://github.com/fannheyward) in https://github.com/charliermarsh/ruff/pull/464
* Implement B017 (no assertRaises(Exception)) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/467
* Implement B002 (unary prefix increment) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/468

## New Contributors
* [@youknowone](https://github.com/youknowone) made their first contribution in https://github.com/charliermarsh/ruff/pull/462

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.82...v0.0.83

[Changes][v0.0.83]


<a name="v0.0.82"></a>
# [v0.0.82](https://github.com/astral-sh/ruff/releases/tag/v0.0.82) - 21 Oct 2022

## What's Changed
* Implement N807 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/456
* Implement N811, 812, 813, 814, and 817 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/457


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.81...v0.0.82

[Changes][v0.0.82]


<a name="v0.0.81"></a>
# [v0.0.81](https://github.com/astral-sh/ruff/releases/tag/v0.0.81) - 18 Oct 2022

## What's Changed
* Implement autofix support for D214, D405, D406, and D416 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/450
* Enable autofix for over- and under-indented docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/451


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.80...v0.0.81

[Changes][v0.0.81]


<a name="v0.0.80"></a>
# [v0.0.80](https://github.com/astral-sh/ruff/releases/tag/v0.0.80) - 17 Oct 2022

## What's Changed
* Update RustPython to get main versions of end_location etc. by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/445
* Split checks and plugins into source-related modules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/447
* Implement autofix for more docstring-related rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/448
* Break up autofix/fixes.rs by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/449


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.79...v0.0.80

[Changes][v0.0.80]


<a name="v0.0.79"></a>
# [v0.0.79](https://github.com/astral-sh/ruff/releases/tag/v0.0.79) - 17 Oct 2022



[Changes][v0.0.79]


<a name="v0.0.78"></a>
# [v0.0.78](https://github.com/astral-sh/ruff/releases/tag/v0.0.78) - 17 Oct 2022

## What's Changed
* Implement autofix for newline-related docstring rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/441
* Implement autofixes for more docstring rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/442
* Re-add the fix icon to README.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/443
* Tweak messages for flake8-comprehensions rules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/444


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.77...v0.0.78

[Changes][v0.0.78]


<a name="v0.0.77"></a>
# [v0.0.77](https://github.com/astral-sh/ruff/releases/tag/v0.0.77) - 16 Oct 2022

## What's Changed
* Implement N801 ~ N805 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/439
* Remove offsets hacks for docstring parsing logic by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/440


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.76...v0.0.77

[Changes][v0.0.77]


<a name="v0.0.76"></a>
# [v0.0.76](https://github.com/astral-sh/ruff/releases/tag/v0.0.76) - 15 Oct 2022

## What's Changed
* Avoid checking for updates when executing via stdin by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/433
* Add initial wasm32-wasi support by [@konstin](https://github.com/konstin) in https://github.com/charliermarsh/ruff/pull/416
* Fix C401 and C402 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/435
* Remove checkmark from rule table by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/436
* Break rules table into sections by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/437

## New Contributors
* [@konstin](https://github.com/konstin) made their first contribution in https://github.com/charliermarsh/ruff/pull/416

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.75...v0.0.76

[Changes][v0.0.76]


<a name="v0.0.75"></a>
# [v0.0.75](https://github.com/astral-sh/ruff/releases/tag/v0.0.75) - 14 Oct 2022

## What's Changed
* Implement D206, D207, and D208 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/429
* Handle multi-byte chars in SourceCodeLocator by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/431


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.74...v0.0.75

[Changes][v0.0.75]


<a name="v0.0.74"></a>
# [v0.0.74](https://github.com/astral-sh/ruff/releases/tag/v0.0.74) - 14 Oct 2022

## What's Changed
* Implement checks for Google-style docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/427
* Implement C417 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/426
* Re-arrange some docstring modules by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/428


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.73...v0.0.74

[Changes][v0.0.74]


<a name="v0.0.73"></a>
# [v0.0.73](https://github.com/astral-sh/ruff/releases/tag/v0.0.73) - 14 Oct 2022

## What's Changed
* Implement C416 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/415
* Implement C411 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/420
* Implement C413 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/421
* Add --config as a command-line option by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/422
* Use test_case for macro-driven check tests by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/424
* Implement docstring argument tracking for NumPy-style docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/425


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.72...v0.0.73

[Changes][v0.0.73]


<a name="v0.0.72"></a>
# [v0.0.72](https://github.com/astral-sh/ruff/releases/tag/v0.0.72) - 13 Oct 2022

## What's Changed
* Implement --fix with stdin by [@fsouza](https://github.com/fsouza) in https://github.com/charliermarsh/ruff/pull/405

## New Contributors
* [@fsouza](https://github.com/fsouza) made their first contribution in https://github.com/charliermarsh/ruff/pull/405

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.71...v0.0.72

[Changes][v0.0.72]


<a name="v0.0.71"></a>
# [v0.0.71](https://github.com/astral-sh/ruff/releases/tag/v0.0.71) - 12 Oct 2022

## What's Changed
* Implement D404 and D418 for pydocstyle by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/409
* Implement D405, D406, D410, D411, and D413 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/411
* Implement D407, D408, D409, D412, and D414 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/413


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.70...v0.0.71

[Changes][v0.0.71]


<a name="v0.0.70"></a>
# [v0.0.70](https://github.com/astral-sh/ruff/releases/tag/v0.0.70) - 12 Oct 2022

## What's Changed
* Implement D402 for pydocstyle by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/403
* Implement D201, D202, D203, D204, and D211 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/404
* Enable definition tracking for docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/407
* Implement docstring visibility checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/408
* Implement C414 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/406


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.69...v0.0.70

[Changes][v0.0.70]


<a name="v0.0.69"></a>
# [v0.0.69](https://github.com/astral-sh/ruff/releases/tag/v0.0.69) - 11 Oct 2022

## What's Changed
* Support linting input from stdin by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/387
* Add fake setup.py by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/399
* Add D212, D213, D300, D403, and D415 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/400
* Skip docstring checks for empty docstrings by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/402


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.68...v0.0.69

[Changes][v0.0.69]


<a name="v0.0.68"></a>
# [v0.0.68](https://github.com/astral-sh/ruff/releases/tag/v0.0.68) - 10 Oct 2022

## What's Changed
* Remove check_ prefix from check utilities by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/393
* Implement docstring tracking by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/394
* Implement D410 (EmptyDocstring) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/395
* Implement D400 (DocstringEndsInNonPeriod) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/396
* Implement D200 (OneLinerDocstring) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/397
* Implement D205, D209, and D210 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/398


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.67...v0.0.68

[Changes][v0.0.68]


<a name="v0.0.67"></a>
# [v0.0.67](https://github.com/astral-sh/ruff/releases/tag/v0.0.67) - 10 Oct 2022

## What's Changed
* Implement B011 from flake8-bugbear by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/390
* Implement B025 from flake8-bugbear by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/391
* Implement B014 from flake8-bugbear by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/392


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.66...v0.0.67

[Changes][v0.0.67]


<a name="v0.0.66"></a>
# [v0.0.66](https://github.com/astral-sh/ruff/releases/tag/v0.0.66) - 10 Oct 2022

## What's Changed
* Implement C409 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/381
* Implement C410 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/382
* Avoid F821 false-positives with NameError by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/386
* Flag unimplemented error codes in M001 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/388


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.65...v0.0.66

[Changes][v0.0.66]


<a name="v0.0.65"></a>
# [v0.0.65](https://github.com/astral-sh/ruff/releases/tag/v0.0.65) - 10 Oct 2022

## What's Changed
* Defer string annotations even when futures annotations are enabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/378
* Rename SPR001 to U008 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/379
* Extend assertEquals check to all deprecated unittest aliases by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/380


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.64...v0.0.65

[Changes][v0.0.65]


<a name="v0.0.64"></a>
# [v0.0.64](https://github.com/astral-sh/ruff/releases/tag/v0.0.64) - 09 Oct 2022

## What's Changed
* Implement C415 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/371
* Fix collapsed message by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/372
* Mark aliased submodule imports as used by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/374
* Treat TypeAlias values as annotations by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/377


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.63...v0.0.64

[Changes][v0.0.64]


<a name="v0.0.63"></a>
# [v0.0.63](https://github.com/astral-sh/ruff/releases/tag/v0.0.63) - 09 Oct 2022

## What's Changed
* Create unified Expr for PEP 604 rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/370


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.62...v0.0.63

[Changes][v0.0.63]


<a name="v0.0.62"></a>
# [v0.0.62](https://github.com/astral-sh/ruff/releases/tag/v0.0.62) - 09 Oct 2022

## What's Changed
* Use strum to iterate over all check codes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/366
* Use strum to facilitate simple enum serialization by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/367
* Implement PEP 585 annotation rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/368
* Implement PEP 604 annotation rewrites by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/369


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.61...v0.0.62

[Changes][v0.0.62]


<a name="v0.0.61"></a>
# [v0.0.61](https://github.com/astral-sh/ruff/releases/tag/v0.0.61) - 08 Oct 2022

## What's Changed
* Update GitHub Actions versions in README by [@StevenMaude](https://github.com/StevenMaude) in https://github.com/charliermarsh/ruff/pull/358
* Add check for W292 by [@cnpryer](https://github.com/cnpryer) in https://github.com/charliermarsh/ruff/pull/339
* Implement C402 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/359
* Add missing C400,C401, and C402 to `CheckCode.from_str` by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/361
* Implement C405 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/362
* Implement C406 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/363
* Implement C408 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/364
* Check newline ending on contents directly by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/365

## New Contributors
* [@StevenMaude](https://github.com/StevenMaude) made their first contribution in https://github.com/charliermarsh/ruff/pull/358
* [@cnpryer](https://github.com/cnpryer) made their first contribution in https://github.com/charliermarsh/ruff/pull/339

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.60...v0.0.61

[Changes][v0.0.61]


<a name="v0.0.60"></a>
# [v0.0.60](https://github.com/astral-sh/ruff/releases/tag/v0.0.60) - 07 Oct 2022

## What's Changed
* Wrap each import in its own backticks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/346
* Implement type(primitive) by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/353
* Rename refactor checks to upgrade checks by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/354
* Warn the user if an explicitly selected check code is ignored by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/356


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.59...v0.0.60

[Changes][v0.0.60]


<a name="v0.0.59"></a>
# [v0.0.59](https://github.com/astral-sh/ruff/releases/tag/v0.0.59) - 07 Oct 2022

## What's Changed
* Implement C400 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/340
* Implement C401 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/343
* Add target Python version as a configurable setting by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/344


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.58...v0.0.59

[Changes][v0.0.59]


<a name="v0.0.58"></a>
# [v0.0.58](https://github.com/astral-sh/ruff/releases/tag/v0.0.58) - 07 Oct 2022

## What's Changed
* Implement C403 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/335
* Enable abspath(__file__) removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/336
* Implement C404 by [@harupy](https://github.com/harupy) in https://github.com/charliermarsh/ruff/pull/338
* Exit 0 if all errors are fixed by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/342


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.57...v0.0.58

[Changes][v0.0.58]


<a name="v0.0.57"></a>
# [v0.0.57](https://github.com/astral-sh/ruff/releases/tag/v0.0.57) - 06 Oct 2022

## What's Changed
* Migrate Checker logic to independent plugins by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/331
* add instructions for setting up cargo insta by [@adriangb](https://github.com/adriangb) in https://github.com/charliermarsh/ruff/pull/334
* support pep593 annotations by [@adriangb](https://github.com/adriangb) in https://github.com/charliermarsh/ruff/pull/333


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.56...v0.0.57

[Changes][v0.0.57]


<a name="v0.0.56"></a>
# [v0.0.56](https://github.com/astral-sh/ruff/releases/tag/v0.0.56) - 05 Oct 2022

## What's Changed
* Add T201 and T203 to string conversion match by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/332


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.55...v0.0.56

[Changes][v0.0.56]


<a name="v0.0.55"></a>
# [v0.0.55](https://github.com/astral-sh/ruff/releases/tag/v0.0.55) - 05 Oct 2022

## What's Changed
* Remove unnecessary Option wrapper from some pyproject::Config fields by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/326
* Enable AST-to-source code generation by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/292
* Support extend-select in pyproject.toml by [@andersk](https://github.com/andersk) in https://github.com/charliermarsh/ruff/pull/327
* Properly combine CLI and pyproject.toml ignores and selects by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/329


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.54...v0.0.55

[Changes][v0.0.55]


<a name="v0.0.54"></a>
# [v0.0.54](https://github.com/astral-sh/ruff/releases/tag/v0.0.54) - 04 Oct 2022

## What's Changed
* Fix the broken link to contribution guidelines by [@ParthS007](https://github.com/ParthS007) in https://github.com/charliermarsh/ruff/pull/321
* Add autofix and default status to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/322
* Only flag super calls in class-function scopes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/323
* Implement __metaclass__ = type removal by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/324

## New Contributors
* [@ParthS007](https://github.com/ParthS007) made their first contribution in https://github.com/charliermarsh/ruff/pull/321

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.53...v0.0.54

[Changes][v0.0.54]


<a name="v0.0.53"></a>
# [v0.0.53](https://github.com/astral-sh/ruff/releases/tag/v0.0.53) - 04 Oct 2022

## What's Changed
* Implement flake8-print by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/308
* Add plugins mention to README by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/309
* Disable plugin-based rules by default by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/318
* Simplify noqa extraction logic by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/320


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.52...v0.0.53

[Changes][v0.0.53]


<a name="v0.0.52"></a>
# [v0.0.52](https://github.com/astral-sh/ruff/releases/tag/v0.0.52) - 03 Oct 2022

## What's Changed
* Handle multi-import lines by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/307


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.51...v0.0.52

[Changes][v0.0.52]


<a name="v0.0.51"></a>
# [v0.0.51](https://github.com/astral-sh/ruff/releases/tag/v0.0.51) - 03 Oct 2022

## What's Changed
* Visit lambda arguments prior to deferral by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/303
* Automatically remove unused imports by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/298


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.50...v0.0.51

[Changes][v0.0.51]


<a name="v0.0.50"></a>
# [v0.0.50](https://github.com/astral-sh/ruff/releases/tag/v0.0.50) - 03 Oct 2022

## What's Changed
* Expose a public 'check' method by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/289
* pre-commit: Validate pyproject.toml by [@cclauss](https://github.com/cclauss) in https://github.com/charliermarsh/ruff/pull/266
* fix: Make assigns to dunder exception for E402. by [@sgryjp](https://github.com/sgryjp) in https://github.com/charliermarsh/ruff/pull/294
* Add end locations to all nodes by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/296
* Add an end location to Check by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/299
* Enable LibCST-based autofixing for SPR001 by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/297
* Avoid falling back to A003 when A001 is disabled by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/302

## New Contributors
* [@cclauss](https://github.com/cclauss) made their first contribution in https://github.com/charliermarsh/ruff/pull/266

**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.49...v0.0.50

[Changes][v0.0.50]


<a name="v0.0.49"></a>
# [v0.0.49](https://github.com/astral-sh/ruff/releases/tag/v0.0.49) - 30 Sep 2022

## What's Changed
* Create CODE_OF_CONDUCT.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/287
* Add CONTRIBUTING.md by [@charliermarsh](https://github.com/charliermarsh) in https://github.com/charliermarsh/ruff/pull/288
* Implement `flake8-super` check by [@sobolevn](https://github.com/sobolevn) in https://github.com/charliermarsh/ruff/pull/291
* Create .editorconfig by [@sobolevn](https://github.com/sobolevn) in https://github.com/charliermarsh/ruff/pull/290


**Full Changelog**: https://github.com/charliermarsh/ruff/compare/v0.0.48...v0.0.49

[Changes][v0.0.49]


<a name="v0.0.48"></a>
# [v0.0.48](https://github.com/astral-sh/ruff/releases/tag/v0.0.48) - 29 Sep 2022



[Changes][v0.0.48]


<a name="v0.0.47"></a>
# [v0.0.47](https://github.com/astral-sh/ruff/releases/tag/v0.0.47) - 29 Sep 2022



[Changes][v0.0.47]


<a name="v0.0.46"></a>
# [v0.0.46](https://github.com/astral-sh/ruff/releases/tag/v0.0.46) - 24 Sep 2022



[Changes][v0.0.46]


<a name="v0.0.45"></a>
# [v0.0.45](https://github.com/astral-sh/ruff/releases/tag/v0.0.45) - 22 Sep 2022



[Changes][v0.0.45]


<a name="v0.0.44"></a>
# [v0.0.44](https://github.com/astral-sh/ruff/releases/tag/v0.0.44) - 21 Sep 2022



[Changes][v0.0.44]


<a name="v0.0.43"></a>
# [v0.0.43](https://github.com/astral-sh/ruff/releases/tag/v0.0.43) - 20 Sep 2022



[Changes][v0.0.43]


<a name="v0.0.42"></a>
# [v0.0.42](https://github.com/astral-sh/ruff/releases/tag/v0.0.42) - 20 Sep 2022



[Changes][v0.0.42]


<a name="v0.0.40"></a>
# [v0.0.40](https://github.com/astral-sh/ruff/releases/tag/v0.0.40) - 16 Sep 2022



[Changes][v0.0.40]


<a name="v0.0.39"></a>
# [v0.0.39](https://github.com/astral-sh/ruff/releases/tag/v0.0.39) - 16 Sep 2022



[Changes][v0.0.39]


<a name="v0.0.38"></a>
# [v0.0.38](https://github.com/astral-sh/ruff/releases/tag/v0.0.38) - 15 Sep 2022



[Changes][v0.0.38]


<a name="v0.0.37"></a>
# [v0.0.37](https://github.com/astral-sh/ruff/releases/tag/v0.0.37) - 13 Sep 2022



[Changes][v0.0.37]


<a name="v0.0.36"></a>
# [v0.0.36](https://github.com/astral-sh/ruff/releases/tag/v0.0.36) - 12 Sep 2022



[Changes][v0.0.36]


<a name="v0.0.35"></a>
# [v0.0.35](https://github.com/astral-sh/ruff/releases/tag/v0.0.35) - 12 Sep 2022



[Changes][v0.0.35]


<a name="v0.0.34"></a>
# [v0.0.34](https://github.com/astral-sh/ruff/releases/tag/v0.0.34) - 11 Sep 2022



[Changes][v0.0.34]


<a name="v0.0.33"></a>
# [v0.0.33](https://github.com/astral-sh/ruff/releases/tag/v0.0.33) - 11 Sep 2022



[Changes][v0.0.33]


<a name="v0.0.32"></a>
# [v0.0.32](https://github.com/astral-sh/ruff/releases/tag/v0.0.32) - 10 Sep 2022



[Changes][v0.0.32]


<a name="v0.0.31"></a>
# [v0.0.31](https://github.com/astral-sh/ruff/releases/tag/v0.0.31) - 10 Sep 2022



[Changes][v0.0.31]


<a name="v0.0.30"></a>
# [v0.0.30](https://github.com/astral-sh/ruff/releases/tag/v0.0.30) - 08 Sep 2022



[Changes][v0.0.30]


<a name="v0.0.29"></a>
# [v0.0.29](https://github.com/astral-sh/ruff/releases/tag/v0.0.29) - 07 Sep 2022



[Changes][v0.0.29]


<a name="v0.0.28"></a>
# [v0.0.28](https://github.com/astral-sh/ruff/releases/tag/v0.0.28) - 06 Sep 2022



[Changes][v0.0.28]


<a name="v0.0.27"></a>
# [v0.0.27](https://github.com/astral-sh/ruff/releases/tag/v0.0.27) - 06 Sep 2022



[Changes][v0.0.27]


<a name="v0.0.26"></a>
# [v0.0.26](https://github.com/astral-sh/ruff/releases/tag/v0.0.26) - 05 Sep 2022



[Changes][v0.0.26]


<a name="v0.0.25"></a>
# [v0.0.25](https://github.com/astral-sh/ruff/releases/tag/v0.0.25) - 03 Sep 2022



[Changes][v0.0.25]


<a name="v0.0.24"></a>
# [v0.0.24](https://github.com/astral-sh/ruff/releases/tag/v0.0.24) - 02 Sep 2022



[Changes][v0.0.24]


<a name="v0.0.23"></a>
# [v0.0.23](https://github.com/astral-sh/ruff/releases/tag/v0.0.23) - 01 Sep 2022



[Changes][v0.0.23]


<a name="v0.0.22"></a>
# [v0.0.22](https://github.com/astral-sh/ruff/releases/tag/v0.0.22) - 31 Aug 2022



[Changes][v0.0.22]


<a name="v0.0.21"></a>
# [v0.0.21](https://github.com/astral-sh/ruff/releases/tag/v0.0.21) - 31 Aug 2022



[Changes][v0.0.21]


<a name="v0.0.20"></a>
# [v0.0.20](https://github.com/astral-sh/ruff/releases/tag/v0.0.20) - 30 Aug 2022



[Changes][v0.0.20]


<a name="v0.0.19"></a>
# [v0.0.19](https://github.com/astral-sh/ruff/releases/tag/v0.0.19) - 30 Aug 2022



[Changes][v0.0.19]


<a name="v0.0.18"></a>
# [v0.0.18](https://github.com/astral-sh/ruff/releases/tag/v0.0.18) - 29 Aug 2022



[Changes][v0.0.18]


[v0.0.280]: https://github.com/astral-sh/ruff/compare/v0.0.279...v0.0.280
[v0.0.279]: https://github.com/astral-sh/ruff/compare/v0.0.278...v0.0.279
[v0.0.278]: https://github.com/astral-sh/ruff/compare/v0.0.277...v0.0.278
[v0.0.277]: https://github.com/astral-sh/ruff/compare/v0.0.276...v0.0.277
[v0.0.276]: https://github.com/astral-sh/ruff/compare/v0.0.275...v0.0.276
[v0.0.275]: https://github.com/astral-sh/ruff/compare/v0.0.274...v0.0.275
[v0.0.274]: https://github.com/astral-sh/ruff/compare/v0.0.273...v0.0.274
[v0.0.273]: https://github.com/astral-sh/ruff/compare/v0.0.272...v0.0.273
[v0.0.272]: https://github.com/astral-sh/ruff/compare/v0.0.271...v0.0.272
[v0.0.271]: https://github.com/astral-sh/ruff/compare/v0.0.270...v0.0.271
[v0.0.270]: https://github.com/astral-sh/ruff/compare/v0.0.269...v0.0.270
[v0.0.269]: https://github.com/astral-sh/ruff/compare/v0.0.268...v0.0.269
[v0.0.268]: https://github.com/astral-sh/ruff/compare/v0.0.267...v0.0.268
[v0.0.267]: https://github.com/astral-sh/ruff/compare/v0.0.266...v0.0.267
[v0.0.266]: https://github.com/astral-sh/ruff/compare/v0.0.265...v0.0.266
[v0.0.265]: https://github.com/astral-sh/ruff/compare/v0.0.264...v0.0.265
[v0.0.264]: https://github.com/astral-sh/ruff/compare/v0.0.263...v0.0.264
[v0.0.263]: https://github.com/astral-sh/ruff/compare/v0.0.262...v0.0.263
[v0.0.262]: https://github.com/astral-sh/ruff/compare/v0.0.261...v0.0.262
[v0.0.261]: https://github.com/astral-sh/ruff/compare/v0.0.260...v0.0.261
[v0.0.260]: https://github.com/astral-sh/ruff/compare/v0.0.259...v0.0.260
[v0.0.259]: https://github.com/astral-sh/ruff/compare/v0.0.258...v0.0.259
[v0.0.258]: https://github.com/astral-sh/ruff/compare/v0.0.257...v0.0.258
[v0.0.257]: https://github.com/astral-sh/ruff/compare/v0.0.256...v0.0.257
[v0.0.256]: https://github.com/astral-sh/ruff/compare/v0.0.255...v0.0.256
[v0.0.255]: https://github.com/astral-sh/ruff/compare/v0.0.254...v0.0.255
[v0.0.254]: https://github.com/astral-sh/ruff/compare/v0.0.253...v0.0.254
[v0.0.253]: https://github.com/astral-sh/ruff/compare/v0.0.252...v0.0.253
[v0.0.252]: https://github.com/astral-sh/ruff/compare/v0.0.251...v0.0.252
[v0.0.251]: https://github.com/astral-sh/ruff/compare/v0.0.250...v0.0.251
[v0.0.250]: https://github.com/astral-sh/ruff/compare/v0.0.249...v0.0.250
[v0.0.249]: https://github.com/astral-sh/ruff/compare/v0.0.248...v0.0.249
[v0.0.248]: https://github.com/astral-sh/ruff/compare/v0.0.247...v0.0.248
[v0.0.247]: https://github.com/astral-sh/ruff/compare/v0.0.246...v0.0.247
[v0.0.246]: https://github.com/astral-sh/ruff/compare/v0.0.245...v0.0.246
[v0.0.245]: https://github.com/astral-sh/ruff/compare/v0.0.244...v0.0.245
[v0.0.244]: https://github.com/astral-sh/ruff/compare/v0.0.243...v0.0.244
[v0.0.243]: https://github.com/astral-sh/ruff/compare/v0.0.242...v0.0.243
[v0.0.242]: https://github.com/astral-sh/ruff/compare/v0.0.241...v0.0.242
[v0.0.241]: https://github.com/astral-sh/ruff/compare/v0.0.240...v0.0.241
[v0.0.240]: https://github.com/astral-sh/ruff/compare/v0.0.239...v0.0.240
[v0.0.239]: https://github.com/astral-sh/ruff/compare/v0.0.238...v0.0.239
[v0.0.238]: https://github.com/astral-sh/ruff/compare/v0.0.237...v0.0.238
[v0.0.237]: https://github.com/astral-sh/ruff/compare/v0.0.236...v0.0.237
[v0.0.236]: https://github.com/astral-sh/ruff/compare/v0.0.235...v0.0.236
[v0.0.235]: https://github.com/astral-sh/ruff/compare/v0.0.234...v0.0.235
[v0.0.234]: https://github.com/astral-sh/ruff/compare/v0.0.233...v0.0.234
[v0.0.233]: https://github.com/astral-sh/ruff/compare/v0.0.232...v0.0.233
[v0.0.232]: https://github.com/astral-sh/ruff/compare/v0.0.231...v0.0.232
[v0.0.231]: https://github.com/astral-sh/ruff/compare/v0.0.230...v0.0.231
[v0.0.230]: https://github.com/astral-sh/ruff/compare/v0.0.229...v0.0.230
[v0.0.229]: https://github.com/astral-sh/ruff/compare/v0.0.228...v0.0.229
[v0.0.228]: https://github.com/astral-sh/ruff/compare/v0.0.227...v0.0.228
[v0.0.227]: https://github.com/astral-sh/ruff/compare/v0.0.226...v0.0.227
[v0.0.226]: https://github.com/astral-sh/ruff/compare/v0.0.225...v0.0.226
[v0.0.225]: https://github.com/astral-sh/ruff/compare/v0.0.224...v0.0.225
[v0.0.224]: https://github.com/astral-sh/ruff/compare/v0.0.223...v0.0.224
[v0.0.223]: https://github.com/astral-sh/ruff/compare/v0.0.222...v0.0.223
[v0.0.222]: https://github.com/astral-sh/ruff/compare/v0.0.221...v0.0.222
[v0.0.221]: https://github.com/astral-sh/ruff/compare/v0.0.220...v0.0.221
[v0.0.220]: https://github.com/astral-sh/ruff/compare/v0.0.219...v0.0.220
[v0.0.219]: https://github.com/astral-sh/ruff/compare/v0.0.218...v0.0.219
[v0.0.218]: https://github.com/astral-sh/ruff/compare/v0.0.217...v0.0.218
[v0.0.217]: https://github.com/astral-sh/ruff/compare/v0.0.216...v0.0.217
[v0.0.216]: https://github.com/astral-sh/ruff/compare/v0.0.215...v0.0.216
[v0.0.215]: https://github.com/astral-sh/ruff/compare/v0.0.214...v0.0.215
[v0.0.214]: https://github.com/astral-sh/ruff/compare/v0.0.213...v0.0.214
[v0.0.213]: https://github.com/astral-sh/ruff/compare/v0.0.212...v0.0.213
[v0.0.212]: https://github.com/astral-sh/ruff/compare/v0.0.211...v0.0.212
[v0.0.211]: https://github.com/astral-sh/ruff/compare/v0.0.210...v0.0.211
[v0.0.210]: https://github.com/astral-sh/ruff/compare/v0.0.209...v0.0.210
[v0.0.209]: https://github.com/astral-sh/ruff/compare/v0.0.208...v0.0.209
[v0.0.208]: https://github.com/astral-sh/ruff/compare/v0.0.207...v0.0.208
[v0.0.207]: https://github.com/astral-sh/ruff/compare/v0.0.206...v0.0.207
[v0.0.206]: https://github.com/astral-sh/ruff/compare/v0.0.205...v0.0.206
[v0.0.205]: https://github.com/astral-sh/ruff/compare/v0.0.204...v0.0.205
[v0.0.204]: https://github.com/astral-sh/ruff/compare/v0.0.203...v0.0.204
[v0.0.203]: https://github.com/astral-sh/ruff/compare/v0.0.202...v0.0.203
[v0.0.202]: https://github.com/astral-sh/ruff/compare/v0.0.201...v0.0.202
[v0.0.201]: https://github.com/astral-sh/ruff/compare/v0.0.200...v0.0.201
[v0.0.200]: https://github.com/astral-sh/ruff/compare/v0.0.199...v0.0.200
[v0.0.199]: https://github.com/astral-sh/ruff/compare/v0.0.198...v0.0.199
[v0.0.198]: https://github.com/astral-sh/ruff/compare/v0.0.196...v0.0.198
[v0.0.196]: https://github.com/astral-sh/ruff/compare/v0.0.195...v0.0.196
[v0.0.195]: https://github.com/astral-sh/ruff/compare/v0.0.194...v0.0.195
[v0.0.194]: https://github.com/astral-sh/ruff/compare/v0.0.193...v0.0.194
[v0.0.193]: https://github.com/astral-sh/ruff/compare/v0.0.192...v0.0.193
[v0.0.192]: https://github.com/astral-sh/ruff/compare/v0.0.191...v0.0.192
[v0.0.191]: https://github.com/astral-sh/ruff/compare/v0.0.190...v0.0.191
[v0.0.190]: https://github.com/astral-sh/ruff/compare/v0.0.189...v0.0.190
[v0.0.189]: https://github.com/astral-sh/ruff/compare/v0.0.188...v0.0.189
[v0.0.188]: https://github.com/astral-sh/ruff/compare/v0.0.187...v0.0.188
[v0.0.187]: https://github.com/astral-sh/ruff/compare/v0.0.186...v0.0.187
[v0.0.186]: https://github.com/astral-sh/ruff/compare/v0.0.185...v0.0.186
[v0.0.185]: https://github.com/astral-sh/ruff/compare/v0.0.184...v0.0.185
[v0.0.184]: https://github.com/astral-sh/ruff/compare/v0.0.183...v0.0.184
[v0.0.183]: https://github.com/astral-sh/ruff/compare/v0.0.182...v0.0.183
[v0.0.182]: https://github.com/astral-sh/ruff/compare/v0.0.181...v0.0.182
[v0.0.181]: https://github.com/astral-sh/ruff/compare/v0.0.180...v0.0.181
[v0.0.180]: https://github.com/astral-sh/ruff/compare/v0.0.179...v0.0.180
[v0.0.179]: https://github.com/astral-sh/ruff/compare/v0.0.178...v0.0.179
[v0.0.178]: https://github.com/astral-sh/ruff/compare/v0.0.177...v0.0.178
[v0.0.177]: https://github.com/astral-sh/ruff/compare/v0.0.176...v0.0.177
[v0.0.176]: https://github.com/astral-sh/ruff/compare/v0.0.175...v0.0.176
[v0.0.175]: https://github.com/astral-sh/ruff/compare/v0.0.174...v0.0.175
[v0.0.174]: https://github.com/astral-sh/ruff/compare/v0.0.173...v0.0.174
[v0.0.173]: https://github.com/astral-sh/ruff/compare/v0.0.172...v0.0.173
[v0.0.172]: https://github.com/astral-sh/ruff/compare/v0.0.171...v0.0.172
[v0.0.171]: https://github.com/astral-sh/ruff/compare/v0.0.170...v0.0.171
[v0.0.170]: https://github.com/astral-sh/ruff/compare/v0.0.169...v0.0.170
[v0.0.169]: https://github.com/astral-sh/ruff/compare/v0.0.168...v0.0.169
[v0.0.168]: https://github.com/astral-sh/ruff/compare/v0.0.167...v0.0.168
[v0.0.167]: https://github.com/astral-sh/ruff/compare/v0.0.166...v0.0.167
[v0.0.166]: https://github.com/astral-sh/ruff/compare/v0.0.165...v0.0.166
[v0.0.165]: https://github.com/astral-sh/ruff/compare/v0.0.164...v0.0.165
[v0.0.164]: https://github.com/astral-sh/ruff/compare/v0.0.163...v0.0.164
[v0.0.163]: https://github.com/astral-sh/ruff/compare/v0.0.162...v0.0.163
[v0.0.162]: https://github.com/astral-sh/ruff/compare/v0.0.161...v0.0.162
[v0.0.161]: https://github.com/astral-sh/ruff/compare/v0.0.160...v0.0.161
[v0.0.160]: https://github.com/astral-sh/ruff/compare/v0.0.159...v0.0.160
[v0.0.159]: https://github.com/astral-sh/ruff/compare/v0.0.158...v0.0.159
[v0.0.158]: https://github.com/astral-sh/ruff/compare/v0.0.157...v0.0.158
[v0.0.157]: https://github.com/astral-sh/ruff/compare/v0.0.156...v0.0.157
[v0.0.156]: https://github.com/astral-sh/ruff/compare/v0.0.155...v0.0.156
[v0.0.155]: https://github.com/astral-sh/ruff/compare/v0.0.154...v0.0.155
[v0.0.154]: https://github.com/astral-sh/ruff/compare/v0.0.153...v0.0.154
[v0.0.153]: https://github.com/astral-sh/ruff/compare/v0.0.152...v0.0.153
[v0.0.152]: https://github.com/astral-sh/ruff/compare/v0.0.151...v0.0.152
[v0.0.151]: https://github.com/astral-sh/ruff/compare/v0.0.150...v0.0.151
[v0.0.150]: https://github.com/astral-sh/ruff/compare/v0.0.149...v0.0.150
[v0.0.149]: https://github.com/astral-sh/ruff/compare/v0.0.148...v0.0.149
[v0.0.148]: https://github.com/astral-sh/ruff/compare/v0.0.147...v0.0.148
[v0.0.147]: https://github.com/astral-sh/ruff/compare/v0.0.146...v0.0.147
[v0.0.146]: https://github.com/astral-sh/ruff/compare/v0.0.145...v0.0.146
[v0.0.145]: https://github.com/astral-sh/ruff/compare/v0.0.144...v0.0.145
[v0.0.144]: https://github.com/astral-sh/ruff/compare/v0.0.143...v0.0.144
[v0.0.143]: https://github.com/astral-sh/ruff/compare/v0.0.142...v0.0.143
[v0.0.142]: https://github.com/astral-sh/ruff/compare/v0.0.141...v0.0.142
[v0.0.141]: https://github.com/astral-sh/ruff/compare/v0.0.140...v0.0.141
[v0.0.140]: https://github.com/astral-sh/ruff/compare/v0.0.139...v0.0.140
[v0.0.139]: https://github.com/astral-sh/ruff/compare/v0.0.138...v0.0.139
[v0.0.138]: https://github.com/astral-sh/ruff/compare/v0.0.137...v0.0.138
[v0.0.137]: https://github.com/astral-sh/ruff/compare/v0.0.136...v0.0.137
[v0.0.136]: https://github.com/astral-sh/ruff/compare/v0.0.135...v0.0.136
[v0.0.135]: https://github.com/astral-sh/ruff/compare/v0.0.134...v0.0.135
[v0.0.134]: https://github.com/astral-sh/ruff/compare/v0.0.133...v0.0.134
[v0.0.133]: https://github.com/astral-sh/ruff/compare/v0.0.132...v0.0.133
[v0.0.132]: https://github.com/astral-sh/ruff/compare/v0.0.131...v0.0.132
[v0.0.131]: https://github.com/astral-sh/ruff/compare/v0.0.130...v0.0.131
[v0.0.130]: https://github.com/astral-sh/ruff/compare/v0.0.129...v0.0.130
[v0.0.129]: https://github.com/astral-sh/ruff/compare/v0.0.128...v0.0.129
[v0.0.128]: https://github.com/astral-sh/ruff/compare/v0.0.127...v0.0.128
[v0.0.127]: https://github.com/astral-sh/ruff/compare/v0.0.126...v0.0.127
[v0.0.126]: https://github.com/astral-sh/ruff/compare/v0.0.125...v0.0.126
[v0.0.125]: https://github.com/astral-sh/ruff/compare/v0.0.124...v0.0.125
[v0.0.124]: https://github.com/astral-sh/ruff/compare/v0.0.123...v0.0.124
[v0.0.123]: https://github.com/astral-sh/ruff/compare/v0.0.122...v0.0.123
[v0.0.122]: https://github.com/astral-sh/ruff/compare/v0.0.121...v0.0.122
[v0.0.121]: https://github.com/astral-sh/ruff/compare/v0.0.120...v0.0.121
[v0.0.120]: https://github.com/astral-sh/ruff/compare/v0.0.119...v0.0.120
[v0.0.119]: https://github.com/astral-sh/ruff/compare/v0.0.118...v0.0.119
[v0.0.118]: https://github.com/astral-sh/ruff/compare/v0.0.117...v0.0.118
[v0.0.117]: https://github.com/astral-sh/ruff/compare/v0.0.116...v0.0.117
[v0.0.116]: https://github.com/astral-sh/ruff/compare/v0.0.115...v0.0.116
[v0.0.115]: https://github.com/astral-sh/ruff/compare/v0.0.114...v0.0.115
[v0.0.114]: https://github.com/astral-sh/ruff/compare/v0.0.113...v0.0.114
[v0.0.113]: https://github.com/astral-sh/ruff/compare/v0.0.112...v0.0.113
[v0.0.112]: https://github.com/astral-sh/ruff/compare/v0.0.111...v0.0.112
[v0.0.111]: https://github.com/astral-sh/ruff/compare/v0.0.110...v0.0.111
[v0.0.110]: https://github.com/astral-sh/ruff/compare/v0.0.109...v0.0.110
[v0.0.109]: https://github.com/astral-sh/ruff/compare/v0.0.108...v0.0.109
[v0.0.108]: https://github.com/astral-sh/ruff/compare/v0.0.107...v0.0.108
[v0.0.107]: https://github.com/astral-sh/ruff/compare/v0.0.106...v0.0.107
[v0.0.106]: https://github.com/astral-sh/ruff/compare/v0.0.105...v0.0.106
[v0.0.105]: https://github.com/astral-sh/ruff/compare/v0.0.104...v0.0.105
[v0.0.104]: https://github.com/astral-sh/ruff/compare/v0.0.103...v0.0.104
[v0.0.103]: https://github.com/astral-sh/ruff/compare/v0.0.102...v0.0.103
[v0.0.102]: https://github.com/astral-sh/ruff/compare/v0.0.100...v0.0.102
[v0.0.100]: https://github.com/astral-sh/ruff/compare/v0.0.99...v0.0.100
[v0.0.99]: https://github.com/astral-sh/ruff/compare/v0.0.98...v0.0.99
[v0.0.98]: https://github.com/astral-sh/ruff/compare/v0.0.97...v0.0.98
[v0.0.97]: https://github.com/astral-sh/ruff/compare/v0.0.96...v0.0.97
[v0.0.96]: https://github.com/astral-sh/ruff/compare/v0.0.95...v0.0.96
[v0.0.95]: https://github.com/astral-sh/ruff/compare/v0.0.94...v0.0.95
[v0.0.94]: https://github.com/astral-sh/ruff/compare/v0.0.93...v0.0.94
[v0.0.93]: https://github.com/astral-sh/ruff/compare/v0.0.92...v0.0.93
[v0.0.92]: https://github.com/astral-sh/ruff/compare/v0.0.91...v0.0.92
[v0.0.91]: https://github.com/astral-sh/ruff/compare/v0.0.90...v0.0.91
[v0.0.90]: https://github.com/astral-sh/ruff/compare/v0.0.89...v0.0.90
[v0.0.89]: https://github.com/astral-sh/ruff/compare/v0.0.88...v0.0.89
[v0.0.88]: https://github.com/astral-sh/ruff/compare/v0.0.87...v0.0.88
[v0.0.87]: https://github.com/astral-sh/ruff/compare/v0.0.86...v0.0.87
[v0.0.86]: https://github.com/astral-sh/ruff/compare/v0.0.85...v0.0.86
[v0.0.85]: https://github.com/astral-sh/ruff/compare/v0.0.84...v0.0.85
[v0.0.84]: https://github.com/astral-sh/ruff/compare/v0.0.83...v0.0.84
[v0.0.83]: https://github.com/astral-sh/ruff/compare/v0.0.82...v0.0.83
[v0.0.82]: https://github.com/astral-sh/ruff/compare/v0.0.81...v0.0.82
[v0.0.81]: https://github.com/astral-sh/ruff/compare/v0.0.80...v0.0.81
[v0.0.80]: https://github.com/astral-sh/ruff/compare/v0.0.79...v0.0.80
[v0.0.79]: https://github.com/astral-sh/ruff/compare/v0.0.78...v0.0.79
[v0.0.78]: https://github.com/astral-sh/ruff/compare/v0.0.77...v0.0.78
[v0.0.77]: https://github.com/astral-sh/ruff/compare/v0.0.76...v0.0.77
[v0.0.76]: https://github.com/astral-sh/ruff/compare/v0.0.75...v0.0.76
[v0.0.75]: https://github.com/astral-sh/ruff/compare/v0.0.74...v0.0.75
[v0.0.74]: https://github.com/astral-sh/ruff/compare/v0.0.73...v0.0.74
[v0.0.73]: https://github.com/astral-sh/ruff/compare/v0.0.72...v0.0.73
[v0.0.72]: https://github.com/astral-sh/ruff/compare/v0.0.71...v0.0.72
[v0.0.71]: https://github.com/astral-sh/ruff/compare/v0.0.70...v0.0.71
[v0.0.70]: https://github.com/astral-sh/ruff/compare/v0.0.69...v0.0.70
[v0.0.69]: https://github.com/astral-sh/ruff/compare/v0.0.68...v0.0.69
[v0.0.68]: https://github.com/astral-sh/ruff/compare/v0.0.67...v0.0.68
[v0.0.67]: https://github.com/astral-sh/ruff/compare/v0.0.66...v0.0.67
[v0.0.66]: https://github.com/astral-sh/ruff/compare/v0.0.65...v0.0.66
[v0.0.65]: https://github.com/astral-sh/ruff/compare/v0.0.64...v0.0.65
[v0.0.64]: https://github.com/astral-sh/ruff/compare/v0.0.63...v0.0.64
[v0.0.63]: https://github.com/astral-sh/ruff/compare/v0.0.62...v0.0.63
[v0.0.62]: https://github.com/astral-sh/ruff/compare/v0.0.61...v0.0.62
[v0.0.61]: https://github.com/astral-sh/ruff/compare/v0.0.60...v0.0.61
[v0.0.60]: https://github.com/astral-sh/ruff/compare/v0.0.59...v0.0.60
[v0.0.59]: https://github.com/astral-sh/ruff/compare/v0.0.58...v0.0.59
[v0.0.58]: https://github.com/astral-sh/ruff/compare/v0.0.57...v0.0.58
[v0.0.57]: https://github.com/astral-sh/ruff/compare/v0.0.56...v0.0.57
[v0.0.56]: https://github.com/astral-sh/ruff/compare/v0.0.55...v0.0.56
[v0.0.55]: https://github.com/astral-sh/ruff/compare/v0.0.54...v0.0.55
[v0.0.54]: https://github.com/astral-sh/ruff/compare/v0.0.53...v0.0.54
[v0.0.53]: https://github.com/astral-sh/ruff/compare/v0.0.52...v0.0.53
[v0.0.52]: https://github.com/astral-sh/ruff/compare/v0.0.51...v0.0.52
[v0.0.51]: https://github.com/astral-sh/ruff/compare/v0.0.50...v0.0.51
[v0.0.50]: https://github.com/astral-sh/ruff/compare/v0.0.49...v0.0.50
[v0.0.49]: https://github.com/astral-sh/ruff/compare/v0.0.48...v0.0.49
[v0.0.48]: https://github.com/astral-sh/ruff/compare/v0.0.47...v0.0.48
[v0.0.47]: https://github.com/astral-sh/ruff/compare/v0.0.46...v0.0.47
[v0.0.46]: https://github.com/astral-sh/ruff/compare/v0.0.45...v0.0.46
[v0.0.45]: https://github.com/astral-sh/ruff/compare/v0.0.44...v0.0.45
[v0.0.44]: https://github.com/astral-sh/ruff/compare/v0.0.43...v0.0.44
[v0.0.43]: https://github.com/astral-sh/ruff/compare/v0.0.42...v0.0.43
[v0.0.42]: https://github.com/astral-sh/ruff/compare/v0.0.40...v0.0.42
[v0.0.40]: https://github.com/astral-sh/ruff/compare/v0.0.39...v0.0.40
[v0.0.39]: https://github.com/astral-sh/ruff/compare/v0.0.38...v0.0.39
[v0.0.38]: https://github.com/astral-sh/ruff/compare/v0.0.37...v0.0.38
[v0.0.37]: https://github.com/astral-sh/ruff/compare/v0.0.36...v0.0.37
[v0.0.36]: https://github.com/astral-sh/ruff/compare/v0.0.35...v0.0.36
[v0.0.35]: https://github.com/astral-sh/ruff/compare/v0.0.34...v0.0.35
[v0.0.34]: https://github.com/astral-sh/ruff/compare/v0.0.33...v0.0.34
[v0.0.33]: https://github.com/astral-sh/ruff/compare/v0.0.32...v0.0.33
[v0.0.32]: https://github.com/astral-sh/ruff/compare/v0.0.31...v0.0.32
[v0.0.31]: https://github.com/astral-sh/ruff/compare/v0.0.30...v0.0.31
[v0.0.30]: https://github.com/astral-sh/ruff/compare/v0.0.29...v0.0.30
[v0.0.29]: https://github.com/astral-sh/ruff/compare/v0.0.28...v0.0.29
[v0.0.28]: https://github.com/astral-sh/ruff/compare/v0.0.27...v0.0.28
[v0.0.27]: https://github.com/astral-sh/ruff/compare/v0.0.26...v0.0.27
[v0.0.26]: https://github.com/astral-sh/ruff/compare/v0.0.25...v0.0.26
[v0.0.25]: https://github.com/astral-sh/ruff/compare/v0.0.24...v0.0.25
[v0.0.24]: https://github.com/astral-sh/ruff/compare/v0.0.23...v0.0.24
[v0.0.23]: https://github.com/astral-sh/ruff/compare/v0.0.22...v0.0.23
[v0.0.22]: https://github.com/astral-sh/ruff/compare/v0.0.21...v0.0.22
[v0.0.21]: https://github.com/astral-sh/ruff/compare/v0.0.20...v0.0.21
[v0.0.20]: https://github.com/astral-sh/ruff/compare/v0.0.19...v0.0.20
[v0.0.19]: https://github.com/astral-sh/ruff/compare/v0.0.18...v0.0.19
[v0.0.18]: https://github.com/astral-sh/ruff/tree/v0.0.18

<!-- Generated by https://github.com/rhysd/changelog-from-release v3.7.1 -->
