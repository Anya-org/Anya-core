/** 
 * This is a simplified example of how to use the documentation duplication detection 
 * feature in the Source of Truth Registry.
 */

import { SourceOfTruthRegistry } from "../registry";
import * as path from "path";

// Initialize the registry
async function initializeRegistry() {
    const registry = await SourceOfTruthRegistry.initialize({
        repositoryRoot: process.cwd(),
        enableBlockchainAnchoring: false,
        enableDocumentationDuplication: true
    });

    return registry;
}

// Example of scanning for documentation duplication
async function detectDocumentationDuplication() {
    const registry = await initializeRegistry();

    // Configure scan options
    const options = {
        scanPath: path.join(process.cwd(), "docs"),
        fileExtensions: ["md", "rst", "txt"],
        similarityThreshold: 0.85,
        ignorePatterns: ["**/node_modules/**", "**/dist/**"],
        outputFormat: "markdown"
    };

    // Run the scan
    console.log("Scanning for documentation duplication...");
    const results = await registry.scanForDocumentationDuplication(options);

    // Print summary
    console.log(`\nScan complete!`);
    console.log(`Files scanned: ${results.filesScanned}`);
    console.log(`Sections analyzed: ${results.sectionsAnalyzed}`);
    console.log(`Duplication groups found: ${results.duplications.length}\n`);

    // Process results
    if (results.duplications.length > 0) {
        console.log("Duplication groups:");

        results.duplications.forEach((group, index) => {
            console.log(`\nGroup ${index + 1} - Similarity: ${Math.round(group.similarity * 100)}%`);

            group.entries.forEach(entry => {
                console.log(`- File: ${entry.filePath}`);
                console.log(`  Section: ${entry.section}`);
                console.log(`  Snippet: "${entry.contentSnippet.substring(0, 100)}..."`);
            });
        });

        // Example of fixing duplications
        console.log("\nActions to address duplications:");
        console.log("1. Consolidate similar sections into a single source");
        console.log("2. Add cross-references between related content");
        console.log("3. Mark canonical sources in the registry");

        // Example of marking a canonical source
        await registry.markAsCanonical(results.duplications[0].entries[0].filePath);
        console.log("\nMarked first entry as canonical source of truth");
    } else {
        console.log("No duplications found! Documentation is well-organized.");
    }
}

// Example of checking a specific document for duplication
async function checkDocumentForDuplication(documentPath: string, content: string) {
    const registry = await initializeRegistry();

    const results = await registry.checkDocumentationDuplication(documentPath, content);

    if (results.isDuplicate) {
        console.log(`Document "${documentPath}" appears to duplicate content from:`);
        results.similarDocuments.forEach(doc => {
            console.log(`- ${doc.filePath} (${Math.round(doc.similarityScore * 100)}% similar)`);
        });
        return false;
    } else {
        console.log(`Document "${documentPath}" contains unique content`);
        return true;
    }
}

// Run the examples
(async () => {
    try {
        await detectDocumentationDuplication();

        const isUnique = await checkDocumentForDuplication(
            "docs/example.md",
            "# Example Document\nThis is an example document to check for duplication."
        );

        console.log(`Document check result: ${isUnique ? "Unique" : "Duplicate"}`);
    } catch (error) {
        console.error("Error:", error);
    }
})();
