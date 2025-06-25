#!/usr/bin/env node

/**
 * DAO GitHub Contribution Tracker
 * 
 * This script integrates GitHub CLI authentication with the DAO token contract for contribution tracking.
 * It records GitHub activities of contributors and maps them to contribution points in the DAO system.
 * 
 * Features:
 * 1. Tracks GitHub activity using GitHub CLI
 * 2. Maps activity to contribution points
 * 3. Integrates with DAO token economics
 * 4. Aligns with Web5 and Lightning authentication flows
 * 5. Supports full history tracking on first run
 * 6. Supports different time periods (30 days, quarter, year)
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const { performance } = require('perf_hooks');

// Import GitHub auth helper
const githubAuth = require('../../scripts/common/github-auth');

// Parse command line arguments for automation flags
const cliArgs = process.argv.slice(2);
const parsedArgs = githubAuth.parseGitHubCliArgs(process.argv);

// Additional CLI arguments
let trackingPeriod = '30days'; // Default tracking period
let forceFullHistory = false;

// Process custom args
for (const arg of cliArgs) {
    if (arg === '--full-history') {
        forceFullHistory = true;
    } else if (arg.startsWith('--period=')) {
        trackingPeriod = arg.split('=')[1];
    }
}

// Configuration
const config = {
    repoOwner: process.env.MCP_GITHUB_DEFAULT_OWNER || 'anya-org',
    repoName: process.env.MCP_GITHUB_DEFAULT_REPO || 'anya-core',
    contributionConfig: path.join(__dirname, '../config/contribution_points.json'),
    outputFile: path.join(__dirname, '../data/contribution_tracking.json'),
    historySummaryFile: path.join(__dirname, '../data/contribution_history.json'),
    trackingDb: path.join(__dirname, '../data/tracking_metadata.json'),
    autoRun: parsedArgs.autoRun,
    yesAll: parsedArgs.yesAll,
    trackingPeriod: trackingPeriod,
    forceFullHistory: forceFullHistory
};

// Contribution point values for different activities
const DEFAULT_CONTRIBUTION_POINTS = {
    commit: 10,
    pullRequest: 30,
    review: 20,
    issue: 5,
    comment: 2
};

// Create necessary directories if they don't exist
function ensureDirectories() {
    const dirs = [
        path.dirname(config.contributionConfig),
        path.dirname(config.outputFile),
        path.dirname(config.trackingDb)
    ];
    
    dirs.forEach(dir => {
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
            console.log(`Created directory: ${dir}`);
        }
    });
}

// Get tracking metadata
function getTrackingMetadata() {
    try {
        if (fs.existsSync(config.trackingDb)) {
            return JSON.parse(fs.readFileSync(config.trackingDb, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load tracking metadata: ${error.message}`);
    }

    // Create default metadata if it doesn't exist
    const metadata = {
        firstRunCompleted: false,
        fullHistoryLastRun: null,
        lastTrackedPeriods: {}
    };
    
    saveTrackingMetadata(metadata);
    
    return metadata;
}

// Save tracking metadata
function saveTrackingMetadata(metadata) {
    ensureDirectories();
    fs.writeFileSync(config.trackingDb, JSON.stringify(metadata, null, 2));
}

// Get contribution points configuration
function getContributionConfig() {
    try {
        if (fs.existsSync(config.contributionConfig)) {
            return JSON.parse(fs.readFileSync(config.contributionConfig, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load contribution config: ${error.message}`);
    }

    // Create default config if it doesn't exist
    ensureDirectories();
    fs.writeFileSync(config.contributionConfig, JSON.stringify(DEFAULT_CONTRIBUTION_POINTS, null, 2));

    return DEFAULT_CONTRIBUTION_POINTS;
}

// Get contribution data
function getContributionData() {
    try {
        if (fs.existsSync(config.outputFile)) {
            return JSON.parse(fs.readFileSync(config.outputFile, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load contribution data: ${error.message}`);
    }

    return {
        lastUpdated: null,
        contributors: {}
    };
}

// Save contribution data
function saveContributionData(data) {
    ensureDirectories();
    
    // Update timestamp
    data.lastUpdated = new Date().toISOString();

    // Save data
    fs.writeFileSync(config.outputFile, JSON.stringify(data, null, 2));
    console.log(`Contribution data saved to ${config.outputFile}`);
}

// Save history summary data
function saveHistorySummary(data) {
    ensureDirectories();
    
    // Save data
    fs.writeFileSync(config.historySummaryFile, JSON.stringify(data, null, 2));
    console.log(`History summary saved to ${config.historySummaryFile}`);
}

// Calculate days from period string
function getDaysFromPeriod(period) {
    switch(period.toLowerCase()) {
        case 'alltime':
        case 'all-time':
        case 'all':
        case 'full':
            return 3650; // ~10 years (effectively all history)
        case 'year':
        case '1year':
        case 'annual':
            return 365;
        case 'quarter':
        case 'quarterly':
            return 91; // ~3 months
        case 'month':
        case '30days':
        case '30d':
            return 30;
        case 'week':
        case '7days':
        case '7d':
            return 7;
        default:
            // Try to parse a number of days
            const days = parseInt(period);
            if (!isNaN(days) && days > 0) {
                return days;
            }
            console.log(`Invalid period "${period}", defaulting to 30 days`);
            return 30;
    }
}

// Format date for GitHub API
function formatDateForGitHub(daysAgo) {
    const date = new Date();
    date.setDate(date.getDate() - daysAgo);
    return date.toISOString().substring(0, 10); // YYYY-MM-DD format
}

// Get commits for a user with a specific time period
function getCommits(username, days) {
    let allCommits = [];
    
    try {
        // Format date for git command
        let sinceParam = '';
        if (days < 3650) {
            const since = new Date();
            since.setDate(since.getDate() - days);
            sinceParam = `--since="${since.toISOString().split('T')[0]}"`;
        }
        
        // Try using direct GitHub API first (more reliable for commit data)
        console.log("Fetching commits page 1...");
        
        // Build appropriate API command based on time period
        let apiCommand;
        if (days < 3650) {
            const since = new Date();
            since.setDate(since.getDate() - days);
            apiCommand = `gh api repos/${config.repoOwner}/${config.repoName}/commits` +
                        `?author=${username}&since=${since.toISOString()}&per_page=100`;
        } else {
            apiCommand = `gh api repos/${config.repoOwner}/${config.repoName}/commits` +
                        `?author=${username}&per_page=100`;
        }
        
        const result = execSync(apiCommand, { encoding: 'utf8' });
        allCommits = JSON.parse(result);
        
        console.log(`Found ${allCommits.length} commits via GitHub API`);
    } catch (error) {
        console.log("GitHub API commit query failed. Using git command fallback.");
        
        try {
            // Use git directly as a fallback
            let gitCommand = `git log --author="${username}" --pretty=format:"%h" --no-merges ${sinceParam}`;
            
            const result = execSync(gitCommand, { encoding: 'utf8' });
            const commits = result.split('\n').filter(line => line.trim() !== '');
            
            console.log(`Found ${commits.length} commits via git command fallback`);
            allCommits = commits.map(hash => ({ sha: hash }));
        } catch (gitError) {
            console.error(`Fallback also failed: ${gitError.message}`);
        }
    }
    
    return allCommits;
}

// Get pull requests for a user with a specific time period
function getPullRequests(username, days) {
    // Format date properly for GitHub API
    const sinceStr = formatDateForGitHub(days);
    
    try {
        console.log(`Fetching PRs using direct PR list for ${username}...`);
        // GitHub CLI PR list command - doesn't support date filtering at API level
        // but we can filter the results in JavaScript after fetching
        const listCommand = `gh pr list --repo ${config.repoOwner}/${config.repoName} --author ${username} --limit 100 --json number,title,url,createdAt`;
        
        const result = execSync(listCommand, { encoding: 'utf8' });
        let prs = JSON.parse(result);
        
        // Apply date filter in JavaScript if needed
        if (days < 3650) {
            const cutoffDate = new Date();
            cutoffDate.setDate(cutoffDate.getDate() - days);
            prs = prs.filter(pr => {
                const prDate = new Date(pr.createdAt);
                return prDate >= cutoffDate;
            });
            
            console.log(`Filtered to ${prs.length} PRs since ${sinceStr}`);
        }
        
        return prs;
    } catch (error) {
        console.error(`Error fetching pull requests: ${error.message}`);
        return [];
    }
}

// Get reviews by a user within a time period
function getReviews(username, days) {
    // Format date properly for GitHub API
    const sinceStr = formatDateForGitHub(days);
    
    try {
        console.log(`Fetching reviews for ${username}...`);
        
        // Get PRs with review data
        const command = `gh pr list --repo ${config.repoOwner}/${config.repoName} --limit 100 --json number,reviews,createdAt`;
        const result = execSync(command, { encoding: 'utf8' });
        const prs = JSON.parse(result);
        
        let userReviews = [];
        const cutoffDate = new Date();
        cutoffDate.setDate(cutoffDate.getDate() - days);
        
        // Filter PRs with reviews by the given username and within date range
        prs.forEach(pr => {
            if (pr.reviews && pr.reviews.length > 0) {
                // Apply date filter if not all-time
                if (days < 3650) {
                    const prDate = new Date(pr.createdAt);
                    if (prDate < cutoffDate) {
                        return; // Skip this PR if it's too old
                    }
                }
                
                // Find reviews by this user
                const userReviewsOnPr = pr.reviews.filter(review => 
                    review.author && review.author.login === username
                );
                
                if (userReviewsOnPr.length > 0) {
                    userReviews.push({
                        number: pr.number,
                        reviews: userReviewsOnPr
                    });
                }
            }
        });
        
        console.log(`Found ${userReviews.length} PRs reviewed by ${username}`);
        
        // Extract just a flat list of reviews
        const allReviews = userReviews.flatMap(pr => pr.reviews);
        return allReviews;
    } catch (error) {
        console.error(`Error fetching reviews: ${error.message}`);
        return [];
    }
}

// Calculate contribution points for a user
function calculateContributionPoints(username, days) {
    console.log(`Calculating contribution points for ${username} over the last ${days} days...`);

    const start = performance.now();
    const pointConfig = getContributionConfig();

    // Get user activities
    const commits = getCommits(username, days);
    const pullRequests = getPullRequests(username, days);
    const reviews = getReviews(username, days);

    // Calculate points
    const commitPoints = commits.length * pointConfig.commit;
    const prPoints = pullRequests.length * pointConfig.pullRequest;
    const reviewPoints = reviews.length * pointConfig.review;
    const totalPoints = commitPoints + prPoints + reviewPoints;

    const end = performance.now();
    console.log(`Calculation completed in ${Math.round(end - start)} ms`);

    return {
        username,
        period: days === 3650 ? 'all-time' : `${days}-days`,
        activities: {
            commits: commits.length,
            pullRequests: pullRequests.length,
            reviews: reviews.length
        },
        points: {
            commits: commitPoints,
            pullRequests: prPoints,
            reviews: reviewPoints,
            total: totalPoints
        },
        timestamp: new Date().toISOString()
    };
}

// Main function to track contributions
async function trackContributions() {
    console.log('DAO GitHub Contribution Tracker');
    console.log('==============================');

    try {
        // Ensure directories exist
        ensureDirectories();

        // Check GitHub CLI auth with auto-run support
        if (!githubAuth.isGitHubCliAvailable()) {
            throw new Error('GitHub CLI is not installed. Please install it first.');
        }

        if (!githubAuth.isGitHubAuthenticated(config.autoRun, config.yesAll)) {
            throw new Error('Not authenticated with GitHub CLI. Please run "gh auth login".');
        }

        // Get GitHub auth info
        const authInfo = githubAuth.getGitHubAuthInfo(config.autoRun, config.yesAll);
        console.log(`Authenticated as: ${authInfo.username}`);

        // Get tracking metadata
        const metadata = getTrackingMetadata();
        
        // Determine if we need to do a full history scan
        // Force this to always run on first execution or if explicitly requested
        const needFullHistory = !metadata.firstRunCompleted || config.forceFullHistory;
        
        if (!needFullHistory) {
            console.log('\nFull history scan not required. Use --full-history to force.');
        }
        
        if (needFullHistory) {
            console.log('\nPerforming full contribution history scan (this may take some time)...');
            
            // Calculate full history contribution points
            const fullHistoryContribution = calculateContributionPoints(authInfo.username, 3650);
            
            // Save history data
            const historyData = {
                lastUpdated: new Date().toISOString(),
                contributors: {
                    [authInfo.username]: fullHistoryContribution
                }
            };
            saveHistorySummary(historyData);
            
            // Update metadata
            metadata.firstRunCompleted = true;
            metadata.fullHistoryLastRun = new Date().toISOString();
            saveTrackingMetadata(metadata);
            
            console.log('\nFull History Contribution Summary for ' + authInfo.username + ':');
            console.log(`- Commits: ${fullHistoryContribution.activities.commits} (${fullHistoryContribution.points.commits} points)`);
            console.log(`- Pull Requests: ${fullHistoryContribution.activities.pullRequests} (${fullHistoryContribution.points.pullRequests} points)`);
            console.log(`- Reviews: ${fullHistoryContribution.activities.reviews} (${fullHistoryContribution.points.reviews} points)`);
            console.log(`- Total All-time Points: ${fullHistoryContribution.points.total}`);
        }

        // Calculate current period contribution points based on selected period
        const days = getDaysFromPeriod(config.trackingPeriod);
        
        // Skip if we already did a full history scan and the period is also full history
        if (needFullHistory && days >= 3650) {
            console.log('\nSkipping duplicate period calculation since full history was already processed.');
        } else {
            console.log(`\nTracking contributions for the last ${days} days...`);
            
            // Calculate contribution points for the selected period
            const periodContribution = calculateContributionPoints(authInfo.username, days);
            
            // Retrieve existing contribution data or create new
            const contributionData = getContributionData();
            contributionData.contributors = contributionData.contributors || {};
            
            // Update with new data
            contributionData.contributors[authInfo.username] = periodContribution;
            contributionData.lastUpdated = new Date().toISOString();
            
            // Save data
            saveContributionData(contributionData);
            
            console.log(`\nContribution Tracking Summary for ${authInfo.username}:`);
            console.log(`- Commits: ${periodContribution.activities.commits} (${periodContribution.points.commits} points)`);
            console.log(`- Pull Requests: ${periodContribution.activities.pullRequests} (${periodContribution.points.pullRequests} points)`);
            console.log(`- Reviews: ${periodContribution.activities.reviews} (${periodContribution.points.reviews} points)`);
            console.log(`- Total Points: ${periodContribution.points.total}`);
            
            // Update tracking metadata
            metadata.lastTrackedPeriods[config.trackingPeriod] = new Date().toISOString();
            saveTrackingMetadata(metadata);
        }
        
        // Share DAO integration information
        console.log('\nSuccess! Contribution data has been calculated and stored.');
        console.log('Use this data with the DAO token contract for automatic rewards.');
        
    } catch (error) {
        console.error(`\nError: ${error.message}`);
        process.exit(1);
    }
}

// Execute the main function
trackContributions();
