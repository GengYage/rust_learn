### rust learn
+ assembly
+ uefi

### uefi code run
1. install qemu and edk2-ovmf
```shell
sudo pacman -S qemu-full
sudo pacman -S edk2-ovmf --needed
```
2. install uefi toolchain
```shell
rustup target add x86_64-unknown-uefi
```
3. build
```shell
cargo build --target x86_64-unknown-uefi --example uefi
```
4. copy and mkdir
```shell
cp /usr/share/edk2/x64/OVMF_CODE.fd .
cp /usr/share/edk2/x64/OVMF_VARS.fd .

mkdir -p esp/efi/boot
cp target/x86_64-unknown-uefi/debug/examples/uefi.efi esp/efi/boot/bootx64.efi
```
5. run qemu
```shell
qemu-system-x86_64 -enable-kvm \                                              
    -drive if=pflash,format=raw,readonly=on,file=OVMF_CODE.fd \
    -drive if=pflash,format=raw,readonly=on,file=OVMF_VARS.fd \
    -drive format=raw,file=fat:rw:esp
```