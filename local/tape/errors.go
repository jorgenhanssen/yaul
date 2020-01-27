package tape

import (
	"errors"
	"fmt"
)

var (
	ErrNilValue = errors.New("Requested value is not initialized")
)

func errAddressOutsideMemory(address int, tape *Instance) error {
	return fmt.Errorf("Address is outside memory area. Requested address was %d. Allowed addresses: [%d, %d]",
		address, tape.minAvailableMemoryAddresses(), tape.maxAvailableMemoryAddresses())
}
