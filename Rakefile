require 'os'

tp = "target/release/fm"
target = "fm"

if OS.windows? then
    tp = "target\\release\\fm.exe"
    target = "fm.exe"
end

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
	if File.exists?(target) then
		File.delete(target)
	end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    sh "cargo clean"
end

task :cleanlock do
    if File.exists?("Cargo.lock") then
        File.delete("Cargo.lock")
    end
end
