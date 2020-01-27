package tape

type Instance struct {
	memory []interface{}
}

func New(size int) *Instance {
	return &Instance{
		memory: make([]interface{}, size),
	}
}

func (i *Instance) Read(address int) (interface{}, error) {
	if i.outsideMemory(address) {
		return 0, errAddressOutsideMemory(address, i)
	}
	val := i.memory[address]
	return val, nil
}

func (i *Instance) Write(address int, value interface{}) error {
	if i.outsideMemory(address) {
		return errAddressOutsideMemory(address, i)
	}
	i.memory[address] = value
	return nil
}

func (i *Instance) outsideMemory(address int) bool {
	return address > i.maxAvailableMemoryAddresses() || address < i.minAvailableMemoryAddresses()
}

func (i *Instance) minAvailableMemoryAddresses() int {
	return 0
}

func (i *Instance) maxAvailableMemoryAddresses() int {
	return len(i.memory) - 1
}
