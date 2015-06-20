require 'benchmark'

class NilClass
  def and_then
    nil
  end
end

class Object
  def and_then
    yield self
  end
end

def bench_add1
  res = 0
  2000.times do
    unless res.nil?
      res =
        if res < 1000
          res + 1
        end
    end
  end
  res
end

def bench_add2
  2000.times.reduce(0) do |r|
    r.and_then { |x| x + 1 if x < 1000 }
  end
end

include Benchmark
Benchmark.benchmark(CAPTION, 7, FORMAT, "ns/iter: ") do |bm|
  t = bm.report("add1") { 10_000.times { bench_add1 } }
  [t * (1_000_000_000 / 10_000)]
end

Benchmark.benchmark(CAPTION, 7, FORMAT, "ns/iter: ") do |bm|
  t = bm.report("add2") { 10_000.times { bench_add2 } }
  [t * (1_000_000_000 / 10_000)]
end

#               user     system      total        real
# add1      1.810000   0.020000   1.830000 (  1.880111)
# ns/iter:  181000.0   2000.000   183000.0 (  188011.1)

#               user     system      total        real
# add2      2.820000   0.020000   2.840000 (  2.900522)
# ns/iter:  282000.0   2000.000   284000.0 (  290052.2)
