# NME 0.0.1-beta.159

beta159에서는 브랜치 push의 전체 rustfmt 검사를 유지하면서 pull request의
Format 작업은 해당 pull request가 변경한 Rust 파일만 검사합니다. 기준
브랜치에 이미 있던 오래된 형식 차이가 형식이 올바른 기여를 막지 않도록
합니다.
