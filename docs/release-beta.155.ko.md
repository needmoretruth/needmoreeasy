# NME 0.0.1-beta.155

beta155에서는 한 줄 Python 함수·클래스 본문에도 공유 Python 범위 진단을
적용합니다. `global`·`nonlocal` 충돌, annotation 대상, 잘못된 `nonlocal` 위치와
별표 import에 안정적인 진단을 표시하면서 올바른 중첩 범위와 모듈 수준 순서는
그대로 보존합니다.
