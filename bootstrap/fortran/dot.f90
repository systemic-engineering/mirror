! Provisional location (spec open-decision #1: substrate path).
! The smallest possible flang-FFI proof: a Fortran function callable from Rust.
! No eigendecomposition, no substrate grammar — just prove the pathway.
!
! NOTE (spec open-decision #5: runtime dependency): the nixpkgs
! llvmPackages.flang 21.1.8 package on aarch64-darwin ships the compiler
! but NOT libflang_rt.runtime.a. Any Fortran intrinsic that lowers to a
! runtime call (e.g. SUM, which emits __FortranASumReal8) therefore fails
! to link. This proof deliberately uses an explicit DO loop so the object
! has ZERO Fortran-runtime dependencies and links with nothing but the
! object itself — isolating the FFI pathway from the runtime question.
function dot5(a, b) result(r) bind(c, name="dot5")
  use iso_c_binding
  real(c_double), intent(in) :: a(5), b(5)
  real(c_double) :: r
  integer :: i
  r = 0.0_c_double
  do i = 1, 5
    r = r + a(i) * b(i)
  end do
end function dot5
