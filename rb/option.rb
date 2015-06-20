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

def maybe_inc(x)
  if x < 1000
    x + 1
  end
end

def bench_add1
  res = 0
  2000.times do
    unless res.nil?
      res = maybe_inc(res)
    end
  end
  res
end

def bench_add2
  (0...2000).reduce(0) do |r,_|
    r.and_then { |x| maybe_inc(x) }
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
# add1      2.270000   0.050000   2.320000 (  2.612915)
# ns/iter:  227000.0   5000.000   232000.0 (  261291.5)

#               user     system      total        real
# add2      3.140000   0.040000   3.180000 (  3.343251)
# ns/iter:  314000.0   4000.000   318000.0 (  334325.1)
