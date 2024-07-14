module binary_search
  implicit none
contains

  function find(array, val) result(idx)
    integer, dimension(:), intent(in) :: array
    integer, intent(in) :: val
    integer :: low, high, idx

    low = 1
    high = size(array)

    do while (low <= high)
        idx = (low + high) / 2
        if (val == array(idx)) then
            return
        else if (val < array(idx)) then
            high = idx - 1
        else
            low = idx + 1
        end if
    end do

    idx = -1


  end function

end module
