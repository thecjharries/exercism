local function prod(s, start, stop)
  local p = 1
  for i = start, stop do
    p = p * tonumber(s:sub(i, i))
  end
  return p
end

return function(opts)
    assert(0 <= opts.span and opts.span <= #opts.digits)
    local max = prod(opts.digits, 1, opts.span)
    local current = max
    for index = 2, #opts.digits - opts.span + 1 do
        current = prod(opts.digits, index, index + opts.span - 1)
        if current > max then
            max = current
        end
    end
    return max
end
