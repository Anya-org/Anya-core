package mobile

//#cgo LDFLAGS: -L${SRCDIR}/../../target/release -lanya_bitcoin
//#include "anya_bitcoin.h"
import "C"
import (
    "encoding/json"
    "errors"
)

type BitcoinMobileSDK struct{}

// SPV Verification with Rust core
func (sdk *BitcoinMobileSDK) VerifySPVProof(txHash string) (bool, error) {
    cHash := C.CString(txHash)
    defer C.free(unsafe.Pointer(cHash))
    
    result := C.verify_spv_proof(cHash)
    return bool(result), nil
}

// Lightning Invoice Generation
func (sdk *BitcoinMobileSDK) CreateInvoice(amount int64) (string, error) {
    cAmount := C.long(amount)
    cInvoice := C.create_lightning_invoice(cAmount)
    defer C.free(unsafe.Pointer(cInvoice))
    
    return C.GoString(cInvoice), nil
} 