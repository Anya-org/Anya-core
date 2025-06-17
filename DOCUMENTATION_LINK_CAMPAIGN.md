# Documentation Link Fixing Campaign

*Date: June 17, 2025*

## Overview

This document outlines the systematic approach to address broken links and documentation structure issues in the Anya-core project. This initiative aims to ensure all documentation is properly cross-referenced, accessible, and maintainable.

## Current Status

An analysis of the codebase has identified several broken links and documentation structure issues:

- Broken internal links between markdown files
- Missing reference documents that are linked to from other files
- Inconsistent link formats (relative vs. absolute paths)
- Outdated references to old GitHub repository URLs
- Disconnect between code implementation and documentation references

## Tools and Resources

We have enhanced our documentation validation tooling to address these issues:

1. **Basic Link Checker**: `scripts/link_checker.py`
   - Finds broken relative links in markdown files
   - Fast initial check for CI/CD pipelines

2. **Advanced Link Campaign**: `scripts/link_campaign.py`
   - Finds broken links with intelligent path resolution
   - Suggests fixes based on filename matching
   - Automatically fixes links with high confidence matches
   - Generates detailed reports of issues and changes

3. **Link Mappings**: `scripts/link_mappings.json`
   - Custom mapping file for problematic links
   - Manually define correct paths for links that can't be automatically resolved

4. **GitHub Pages Validator**: `scripts/validate-gh-pages.sh`
   - Validates GitHub Pages configuration and structure
   - Ensures documentation can be properly built and deployed

5. **CI Integration**: `scripts/validate_links_ci.sh` and `.github/workflows/docs-health-check.yml`
   - Integrates all checks into CI/CD pipeline
   - Ensures documentation quality before merges

## Action Plan

### Immediate Actions (Complete by June 20, 2025)

1. **Fix Critical Broken Links**
   - Run `python3 scripts/link_campaign.py --fix` to automatically fix high-confidence broken links
   - Manually review and fix links in the generated report that couldn't be automatically fixed

2. **Add Missing Documentation**
   - Create missing documentation files that are referenced but don't exist
   - Where appropriate, consolidate documentation to reduce redundancy

3. **Update GitHub Workflows**
   - Ensure `docs-link-check.yml` is properly configured
   - Add new `docs-health-check.yml` workflow for comprehensive checks

### Short-term Actions (Complete by June 30, 2025)

1. **Enhance Documentation Structure**
   - Implement consistent navigation between related documents
   - Ensure each document has proper "See Also" sections
   - Add breadcrumbs to improve navigation

2. **Improve CI/CD Integration**
   - Integrate link checking as pre-commit hooks
   - Set up automated pull requests for documentation fixes

3. **Documentation Standards**
   - Create and enforce markdown style guide
   - Implement consistent header structures and metadata

### Long-term Maintenance (Ongoing)

1. **Regular Audits**
   - Schedule monthly documentation audits via GitHub Actions
   - Track documentation health metrics over time

2. **Documentation Testing**
   - Implement automated tests for documentation accuracy
   - Validate code examples in documentation

3. **User Feedback Loop**
   - Add feedback mechanisms to documentation
   - Track common documentation pain points

## Implementation Details

### Link Campaign Script Usage

```bash
# Run in dry-run mode (report only)
python3 scripts/link_campaign.py

# Fix links automatically where possible
python3 scripts/link_campaign.py --fix

# Generate report only
python3 scripts/link_campaign.py --report
```

### Custom Link Mappings

Edit `scripts/link_mappings.json` to add custom link resolutions:

```json
{
  "broken/path.md": "correct/path.md",
  "another/broken.md": "../fixed.md"
}
```

### CI/CD Integration

The GitHub Actions workflow will:

1. Check for broken links on every PR that modifies documentation
2. Generate a report as an artifact for review
3. Block merges if critical documentation issues are found

## Success Metrics

We will track the following metrics to measure the success of this campaign:

1. **Broken Link Count**: Target zero broken internal links
2. **Documentation Coverage**: All code functionality should have documentation
3. **Build Success Rate**: 100% success rate for documentation builds
4. **User Feedback**: Reduced issues related to documentation

## Team Responsibilities

- **Documentation Lead**: Oversee campaign and review progress
- **Developers**: Fix documentation related to their code
- **CI/CD Engineer**: Ensure automated checks are working properly
- **Technical Writers**: Review and improve documentation quality

## References

- [GitHub Pages Documentation](https://docs.github.com/en/pages)
- [MkDocs Documentation](https://www.mkdocs.org/)
- [Markdown Linting Rules](https://github.com/markdownlint/markdownlint/blob/main/docs/RULES.md)
