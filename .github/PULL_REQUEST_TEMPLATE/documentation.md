---
name: Documentation Change
about: Pull request template for documentation changes
title: 'docs: '
labels: documentation
---

## Description

<!--
Please provide a clear description of what this PR changes in the documentation.
If this PR is part of the link fixing campaign, please mention that.
-->

## Type of Documentation Change

<!-- Please check all that apply by replacing [ ] with [x] -->

- [ ] Fix broken links
- [ ] Create missing documentation
- [ ] Update existing documentation
- [ ] Restructure documentation
- [ ] Improve formatting/readability
- [ ] Fix technical inaccuracies
- [ ] Other (please specify)

## Verification

<!-- Please check all that apply by replacing [ ] with [x] -->

- [ ] I have run `scripts/link_checker.py` to verify no broken links were introduced
- [ ] I have run `scripts/validate-gh-pages.sh` to validate GitHub Pages structure
- [ ] All new and modified files follow the [Markdown Style Guide](/docs/standards/MARKDOWN_STYLE_GUIDE.md)
- [ ] Documentation builds correctly with `mkdocs build`

## Screenshots (if applicable)

<!-- If your changes include visual improvements, please include before/after screenshots -->

## Related Issues

<!-- Please link any related issues here -->
- Fixes #(issue)

## Additional Information

<!-- Any additional information that might be helpful for reviewers -->
