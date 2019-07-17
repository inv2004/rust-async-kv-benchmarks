n = 1

request = function()
     n = n + 1
     local r = {}
     local s = tostring(n)
     r[1] = wrk.format("PUT", "/", {}, s..":"..s)
     r[2] = wrk.format("GET", "/", {}, s)
     return table.concat(r)
end
