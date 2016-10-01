require 'os'

pkg = "demo_from_json_0.1-1"

task :default do
    sh "cargo build"
end

task :test do
    sh "target/debug/dpkg-deb-rst.exe -b demo_0.1-1.json"
    puts
    if os.windows then
        sh "tree /F #{pkg}"
    else
        sh "tree #{pkg}"
    end
    puts
    sh "cat #{pkg}/DEBIAN/control"
end
