require 'os'
require 'fileutils'

pkg = "demo_from_json_0.1-1"

task :default do
    sh "cargo build --release"
end

task :test do
    puts
    sh "target/release/dpkg-deb-rst --build demo_0.1-1"
    puts
    sh "target/release/dpkg-deb-rst --build demo_0.1-1.json"
    puts
    if OS.windows? then
        sh "tree /F #{pkg}"
        sh "type #{pkg}\\DEBIAN\\control"
    else
        sh "tree #{pkg}"
        sh "cat #{pkg}/DEBIAN/control"
    end
end

task :clean do
    File.delete("control.tar.gz")
    File.delete("debian-binary")
    FileUtils.rm_rf(pkg)
    FileUtils.rm_rf("target")
end
