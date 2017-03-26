require 'os'
require 'fileutils'

target = "dpkg-deb-rst"
rtarget = "target/release/#{target}"
pkgs = [ "demo_from_json_0.1-1", "demo_from_toml_0.1-1", "demo_from_yaml_0.1-1" ]
exts = [ "json", "toml", "yaml" ]

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
	sh "upx -9 #{rtarget} -o #{target}"
end

task :test do
    puts
    sh "#{rtarget} --help"
    puts
    sh "#{rtarget} --build demo_0.1-1"
    puts
    for i in 0..pkgs.size - 1 do
        sh "#{rtarget} --build demo_0.1-1.#{exts[i]}"
        puts
        if OS.windows? then
            sh "tree /F #{pkgs[i]}"
            sh "type #{pkgs[i]}\\DEBIAN\\control"
        else
            sh "tree #{pkgs[i]}"
            sh "cat #{pkgs[i]}/DEBIAN/control"
        end
        puts
    end
end

task :clean do
    File.delete("control.tar.gz")
    File.delete("debian-binary")
    for i in 0..pkgs.size - 1 do
        FileUtils.rm_rf(pkgs[i])
    end
end

task :cleanall => [:clean] do
    sh "cargo clean"
end

task :cleanlock do
    File.delete("Cargo.lock")
end
