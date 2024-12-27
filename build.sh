sudo rm -rf ./build
docker build -t doryani-build -f cc.Dockerfile .
docker run -v ./build:/build doryani-build

echo "Finished building!"

ls -al build
