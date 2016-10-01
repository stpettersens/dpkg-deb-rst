pkg = "demo_from_json_0.1.1"

task :default do
    sh "cargo build"
end

task :test do
    sh "target/debug/dpkg-deb-rst.exe -b ${pkg}.json"
    sh "tree #{pkg} && cat #{pkg}/DEBIAN/control"
end
