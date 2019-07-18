
-- for patched wrk2 only https://github.com/inv2004/wrk2

function init(x)
    tid = wrk.thread.tindex
end

n = 1

function request()
    n = n + 1
    local s
    if tid == 0 then
        s = tostring(n)
        return wrk.format("PUT", "/", {}, s..":"..s)
    else
        s = tostring(n-1000)
        return wrk.format("GET", "/", {}, s)
    end
end
