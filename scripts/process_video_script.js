// Adding integration script for our 90-second technical video
// with the optimizations requested by the product team

const fs = require('fs');
const path = require('path');

// Configuration
const config = {
    inputScriptPath: '../docs/v1.3_optimized_video_script.md',
    outputDir: '../web/assets/videos',
    videoTitle: 'Anya-core v1.3 Technical Overview',
    slideTransitions: [5, 25, 45, 65, 85], // Transition points in seconds
    featuredTechnologies: [
        'Bitcoin Taproot',
        'Web5 Integration',
        'Source of Truth Registry',
        'Documentation Analysis'
    ]
};

// Function to parse the markdown script
function parseVideoScript(scriptPath) {
    try {
        const content = fs.readFileSync(scriptPath, 'utf8');
        const sections = content.split('##').filter(section => section.trim().length > 0);

        return {
            title: content.split('\n')[0].replace('# ', ''),
            sections: sections.map(section => {
                const lines = section.trim().split('\n');
                const title = lines[0].trim();
                const timing = title.match(/\((\d+:\d+)-(\d+:\d+)\)/);
                const content = lines.slice(1).join('\n').trim();

                return {
                    title: title.replace(/\(\d+:\d+-\d+:\d+\)/, '').trim(),
                    startTime: timing ? timing[1] : null,
                    endTime: timing ? timing[2] : null,
                    content
                };
            })
        };
    } catch (err) {
        console.error('Error parsing video script:', err);
        return null;
    }
}

// Function to generate visualization data
function generateVisualizationData(parsedScript) {
    // This would contain the logic to generate visualization data
    // based on the script sections

    return parsedScript.sections.map(section => {
        // Generate appropriate visualizations based on section content
        const visualType = section.title.includes('BENEFITS') ?
            'chart' : section.title.includes('TECHNICAL') ?
                'code' : 'diagram';

        return {
            ...section,
            visualType,
            dataPoints: extractKeyDataPoints(section.content)
        };
    });
}

// Extract key data points from section content
function extractKeyDataPoints(content) {
    // This is a simplistic extraction - in a real implementation
    // this would use NLP or more sophisticated parsing

    const bulletPoints = content.match(/\*\*(.*?)\*\*/g) || [];
    return bulletPoints.map(point => point.replace(/\*\*/g, ''));
}

// Main function
function processVideoScript() {
    console.log(`Processing video script for ${config.videoTitle}...`);

    const parsedScript = parseVideoScript(config.inputScriptPath);
    if (!parsedScript) return;

    const visualData = generateVisualizationData(parsedScript);

    // Create output directory if it doesn't exist
    if (!fs.existsSync(config.outputDir)) {
        fs.mkdirSync(config.outputDir, { recursive: true });
    }

    // Write visualization data to JSON file
    const outputPath = path.join(config.outputDir, 'video_data.json');
    fs.writeFileSync(outputPath, JSON.stringify({
        title: parsedScript.title,
        duration: '90 seconds',
        sections: visualData,
        transitions: config.slideTransitions,
        featuredTechnologies: config.featuredTechnologies
    }, null, 2));

    console.log(`Video data processed successfully and saved to ${outputPath}`);
}

// Execute the script
processVideoScript();
