#!/bin/sh

arch=$1
fs=$2

contain_x86_64=false
for arg in "$@"
do
	if [ "$arg" = "x86_64" ]; then
		contain_x86_64=true
		break
	fi
done

if [ "$contain_x86_64" = true ]; then
	arch=x86_64
	FILE=testsuits-x86_64-linux-musl
	echo "true"
	if [ ! -e testcases/$FILE ]; then
		wget https://github.com/rcore-os/testsuits-for-oskernel/releases/download/final-x86_64/testsuits-x86_64-linux-musl.tgz
		tar zxvf $FILE.tgz
		mv $FILE testcases/$FILE -f
		rm $FILE.tgz
	fi
	if [ -n "$3" ]; then
		FILE=$3
	fi
else
	if [ "$arch" = "riscv64" ]; then
		FILE=testsuits-riscv64-linux-musl
		if [ ! -e testcases/$FILE ]; then
			wget https://github.com/rcore-os/testsuits-for-oskernel/releases/download/final-20240222/testsuits-riscv64-linux-musl.tgz
			tar zxvf $FILE.tgz
			mv $FILE testcases/$FILE -f
			rm $FILE.tgz
		fi
		if [ -n "$3" ]; then
			FILE=$3
		fi

	elif [ "$arch" = "aarch64" ]; then
		FILE=testsuits-aarch64-linux-musl
		if [ ! -e testcases/$FILE ]; then
			wget https://github.com/rcore-os/testsuits-for-oskernel/releases/download/final-20240222/testsuits-aarch64-linux-musl.tgz
			tar zxvf $FILE.tgz
			mv $FILE testcases/$FILE -f
			rm $FILE.tgz
		fi
		if [ -n "$3" ]; then
			FILE=$3
		fi

	else
		arch=x86_64
		FILE=testsuits-x86_64-linux-musl
		echo "false"
		if [ ! -e testcases/$FILE ]; then
			wget https://github.com/rcore-os/testsuits-for-oskernel/releases/download/final-x86_64/testsuits-x86_64-linux-musl.tgz
			tar zxvf $FILE.tgz
			mv $FILE testcases/$FILE -f
			rm $FILE.tgz
		fi
		if [ -n "$3" ]; then
			FILE=$3
		fi

	fi
fi

rm disk.img
dd if=/dev/zero of=disk.img bs=4M count=30

if [ "$fs" = "ext4" ]; then
	mkfs.ext4 -t ext4 disk.img
else
	fs=fat32
	mkfs.vfat -F 32 disk.img
fi

mkdir -p mnt
sudo mount disk.img mnt

# 根据命令行参数生成对应的测例
echo "Copying $arch $fs $FILE/* to disk"
sudo cp -r ./testcases/$FILE/* ./mnt/
sudo umount mnt
sudo rm -rf mnt
sudo chmod 777 disk.img
