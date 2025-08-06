import { DocumentationDuplication } from '../ts/registry-verification';

/**
 * Component to visualize documentation duplication in the UI
 */
class DuplicationVisualizer extends HTMLElement {
    private _data: DocumentationDuplication[] = [];
    private _threshold: number = 0.8;

    constructor() {
        super();
        this.attachShadow({ mode: 'open' });
        this.render();
    }

    /**
     * Set the duplication data to visualize
     */
    set data(duplicationData: DocumentationDuplication[]) {
        this._data = duplicationData;
        this.render();
    }

    /**
     * Set the similarity threshold for highlighting
     */
    set threshold(value: number) {
        this._threshold = value;
        this.render();
    }

    /**
     * Connect to the Source of Truth Registry
     */
    async connectToRegistry(registryUrl: string): Promise<void> {
        try {
            const response = await fetch(`${registryUrl}/api/documentation-duplications`);
            const data = await response.json();
            this._data = data.duplications;
            this.render();

            this.dispatchEvent(new CustomEvent('connected', {
                detail: { success: true, count: this._data.length }
            }));
        } catch (error) {
            console.error('Failed to connect to registry:', error);
            this.dispatchEvent(new CustomEvent('connected', {
                detail: { success: false, error }
            }));
        }
    }

    /**
     * Render the component
     */
    private render(): void {
        if (!this.shadowRoot) return;

        this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          font-family: system-ui, -apple-system, sans-serif;
          margin: 1rem 0;
        }
        
        .container {
          border: 1px solid #ccc;
          border-radius: 4px;
          padding: 1rem;
        }
        
        .header {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 1rem;
        }
        
        .duplication-list {
          list-style: none;
          padding: 0;
        }
        
        .duplication-item {
          margin-bottom: 1rem;
          padding: 0.75rem;
          border-radius: 4px;
          border-left: 3px solid #5c6bc0;
          background-color: #f5f5f5;
        }
        
        .high-similarity {
          border-left-color: #f44336;
          background-color: #ffebee;
        }
        
        .medium-similarity {
          border-left-color: #ff9800;
          background-color: #fff3e0;
        }
        
        .file-paths {
          font-family: monospace;
          font-size: 0.9em;
          margin: 0.5rem 0;
        }
        
        .snippet {
          background-color: rgba(0, 0, 0, 0.04);
          padding: 0.5rem;
          border-radius: 2px;
          font-family: monospace;
          white-space: pre-wrap;
          margin: 0.5rem 0;
          max-height: 200px;
          overflow-y: auto;
        }
        
        .empty-state {
          text-align: center;
          padding: 2rem;
          color: #666;
        }
      </style>
      
      <div class="container">
        <div class="header">
          <h3>Documentation Duplication Analysis</h3>
          <span>${this._data.length} duplications found</span>
        </div>
        
        ${this.renderDuplicationsContent()}
      </div>
    `;
    }

    /**
     * Render the duplications content
     */
    private renderDuplicationsContent(): string {
        if (this._data.length === 0) {
            return `
        <div class="empty-state">
          <p>No documentation duplications found</p>
        </div>
      `;
        }

        return `
      <ul class="duplication-list">
        ${this._data.map(dup => this.renderDuplicationItem(dup)).join('')}
      </ul>
    `;
    }

    /**
     * Render a single duplication item
     */
    private renderDuplicationItem(duplication: DocumentationDuplication): string {
        const similarityClass = duplication.similarity > 0.9
            ? 'high-similarity'
            : duplication.similarity > 0.7
                ? 'medium-similarity'
                : '';

        return `
      <li class="duplication-item ${similarityClass}">
        <div>
          <strong>Similarity: ${Math.round(duplication.similarity * 100)}%</strong>
        </div>
        
        <div class="file-paths">
          ${duplication.locations.map(loc => `
            <div>${loc.filePath}:${loc.section || 'Whole file'}</div>
          `).join('')}
        </div>
        
        <div class="snippet">
          ${this.escapeHtml(duplication.snippet || 'No snippet available')}
        </div>
      </li>
    `;
    }

    /**
     * Escape HTML to prevent XSS
     */
    private escapeHtml(html: string): string {
        const div = document.createElement('div');
        div.textContent = html;
        return div.innerHTML;
    }
}

// Register the custom element
customElements.define('duplication-visualizer', DuplicationVisualizer);
