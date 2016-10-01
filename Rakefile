stagedir = "demo_from_json_0.1.1"

task :default do
    sh "cargo build"
end

task :test do
    sh "tree demo_from_json_0.1-1 && cat demo_from_json_0.1-1/control"
    sh "tree #{stagedir} && cat #{stagedir}/DEBIAN/control"
end
