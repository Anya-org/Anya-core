## Architecture
```mermaid
flowchart TD
    A[Enterprise Features] --> B{Anya Enterprise Submodule}
    B --> C[Bitcoin Core]
    B --> D[Advanced Security]
    C --> E[BIP-341/342]
    D --> F[FIDO2 HSM]
``` 
