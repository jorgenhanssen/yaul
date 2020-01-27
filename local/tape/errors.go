package tape

import "fmt"

func errAddressOutsideMemory(address int, tape *Instance) error {
	return fmt.Errorf("Address is outside memory area. Requested address was %d. Allowed addresses: [%d, %d]",
		address, tape.minAvailableMemoryAddresses(), tape.maxAvailableMemoryAddresses())
}
