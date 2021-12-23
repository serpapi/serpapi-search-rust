
task :default => [:fmt, :test, :oobt, :release]

task :fmt do 
  sh("cargo fmt")
end

task :clean do
  sh("cargo clean")
end

task :test do 
  sh("cargo test")
end

task :oobt do 
  sh("cargo run --example google_search_example")
end

task :release => [:fmt, :test, :oobt] do 
  sh("cargo publish")
end
