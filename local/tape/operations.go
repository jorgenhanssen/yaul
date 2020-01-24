package tape

func (i *Instance) Read(address int) (interface{}, error) {
	if address >= i.maxAvailableMemoryAddresses() {
		return 0, errAddressOutsideMemory(address, i)
	}
	return i.memory[address], nil
}

func (i *Instance) Write(address int, value interface{}) error {
	if address >= i.maxAvailableMemoryAddresses() {
		return errAddressOutsideMemory(address, i)
	}
	i.memory[address] = value
	return nil
}
