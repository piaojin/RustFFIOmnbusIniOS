#合并各target生成的.a文件到一个.a文件，有第三方的命令库https://github.com/TimNN/cargo-lipo可以达到同样效果。实际过程中在自己的mac模拟器上跑程序时需要包含x86_64-apple-ios，i386-apple-ios架构的.a文件，而真正打包成ipa安装包时并不需要x86_64-apple-ios，i386-apple-ios架构的.a文件，所以需要shell脚本判断是真机跑还是模拟器跑，以达到只打包必要cpu架构的减少.a文件大小的目的。（这个判断脚本不会😑😑😑😑😑😑）
libtool -static -o target/debug/libRust_FFI_Omnbus.a target/x86_64-apple-ios/debug/libRust_FFI_Omnbus.a target/i386-apple-ios/debug/libRust_FFI_Omnbus.a target/armv7s-apple-ios/debug/libRust_FFI_Omnbus.a target/armv7-apple-ios/debug/libRust_FFI_Omnbus.a target/aarch64-apple-ios/debug/libRust_FFI_Omnbus.a

#合并.a文件转.dylib文件,不过目前转换不成功
#g++ -fpic -shared target/debug/libRust_FFI_Omnbus.a -o target/debug/libRust_FFI_Omnbus.dylib

#rm -f target/debug/libRust_FFI_Omnbus.a
#删除个target的.a文件
rm -rf target/x86_64-apple-ios target/i386-apple-ios target/armv7s-apple-ios target/armv7-apple-ios target/aarch64-apple-ios
rm -f libRust_FFI_Omnbus.a
