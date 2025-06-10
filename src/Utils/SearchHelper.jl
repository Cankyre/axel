const stop_flag = Threads.Atomic{Bool}(false)

function is_search_cancelled()
    return stop_flag[]
end

function cancel_search()
    stop_flag[] = true
    return stop_flag[]
end

function reset_stop_flag()
    stop_flag[] = false
    return stop_flag[]
end