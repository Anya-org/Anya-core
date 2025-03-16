# Add search performance test
Measure-Command {
    clarinet run '(contract-call? .dao-core search-proposals
        none
        (some "governance") 
        (list "budget" "funding"))'
} | Export-Csv -Path "perf-search.csv" 