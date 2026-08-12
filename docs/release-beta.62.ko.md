# NME 0.0.1-beta.62

beta.62에서는 네이티브 C 이름공간 검사를 강화합니다.

- 생성된 `limits.h`, `stdio.h`, `stdlib.h`, `string.h` 헤더가 노출하는 매크로·
  typedef·선언을 예약합니다.
- 전처리로 다시 쓰이거나 C 라이브러리 선언과 충돌할 수 있는 식별자를
  거부합니다.
