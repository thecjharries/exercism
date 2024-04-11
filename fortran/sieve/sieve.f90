module sieve
  implicit none

contains

  function primes(limit) result(array)
    integer, intent(in) :: limit
    integer, allocatable :: array(:)
    logical :: sieve(limit)
    integer :: i, j

    sieve = .true.

    do i = 2, int(sqrt(real(limit)))
      if (sieve(i)) then
        do j = i**2, limit, i
          sieve(j) = .false.
        end do
      end if
    end do

    array = pack([(i, i = 2, limit)], sieve(2:))
  end function primes

end module sieve
