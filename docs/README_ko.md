<p align="center">
  <img src="../src-tauri/icons/128x128.png" width="96" alt="Distillery" />
</p>

<h1 align="center">Distillery</h1>

<p align="center">
  <em>생각을 증류하여 지식으로 숙성시키다.</em>
</p>

---

```
  몰팅         몰트 하우스        스틸          대기열          캐스크
  ─────       ───────────      ──────       ──────────      ──────
 [ 생각 ] ──→ [  보관  ] ──→ [  선별  ] ──→ [  증류  ] ──→ [ 지식 ]
               ↑                   │
               └───── 드로우백 ─────┘
```

Distillery는 위스키 증류소의 과정을 빌려 **생각을 정제하는 데스크톱 앱**입니다.

흩어진 결정, 문제, 인사이트, 질문을 **몰트**로 빚고,
**스틸**에 올려 선별한 뒤,
서버가 매일 정오와 자정에 자동으로 증류하여
**캐스크**(지식 그래프)에서 숙성시킵니다.

---

## 증류 과정

### 1. 몰팅 (Malting)

생각을 몰트로 만듭니다. 네 가지 유형 중 하나를 선택하세요.

| 유형 | 설명 |
|------|------|
| **결정** | 확정된 의사결정 |
| **문제** | 해결이 필요한 이슈 |
| **인사이트** | 발견한 통찰 |
| **질문** | 논의가 필요한 질문 |

### 2. 몰트 하우스 (Malt House)

만들어진 몰트를 보관하는 창고. 수정, 삭제, 검색이 가능합니다.

### 3. 스틸 (Still)

증류할 몰트를 선별하는 단계. 준비가 되면 증류 대기열로 보냅니다.

### 4. 드로우백 (Draw Back)

대기열에서 몰트를 빼냅니다. 캐스크에 들어간 몰트는 되돌릴 수 없습니다.

---

## 플로팅 메모 (빠른 몰팅)

메인 앱으로 전환하지 않고 어디서든 생각을 즉시 기록할 수 있는 작은 플로팅 창입니다.
앱은 **시스템 트레이**에 상주하며, 메인 창을 닫아도 백그라운드에서 계속 실행됩니다.

| 플랫폼 | 단축키 | 설정 |
|--------|--------|------|
| **macOS** | `Cmd+Shift+M` | 바로 동작 |
| **Windows** | `Ctrl+Shift+M` | 바로 동작 |
| **Linux (X11)** | `Ctrl+Shift+M` | 바로 동작 |
| **Linux (Wayland)** | 원하는 키 | 컴포지터에서 DBus 명령 바인딩 (아래 참고) |

### Linux Wayland 설정

Wayland는 앱 수준의 글로벌 단축키를 지원하지 않습니다.
대신 앱이 DBus 서비스를 제공하므로, 사용 중인 컴포지터/DE 설정에서 아래 명령을 바인딩하세요:

```bash
dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo
```

<details>
<summary>niri</summary>

`config.kdl`의 `binds` 블록에 추가:

```kdl
Ctrl+Shift+M { spawn "dbus-send" "--session" "--type=method_call" "--dest=com.distillery.App" "/com/distillery/App" "com.distillery.App.ToggleFloatingMemo"; }
```

리로드: `niri msg action reload-config`
</details>

<details>
<summary>Hyprland</summary>

`hyprland.conf`에 추가:

```
bind = CTRL SHIFT, M, exec, dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo
```
</details>

<details>
<summary>GNOME</summary>

```bash
gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/']"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ name 'Distillery 플로팅 메모'
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ command "dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo"
gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/distillery/ binding '<Ctrl><Shift>m'
```
</details>

<details>
<summary>KDE Plasma</summary>

시스템 설정 > 단축키 > 사용자 정의 단축키에서 새 단축키 추가:

- 트리거: `Ctrl+Shift+M`
- 명령: `dbus-send --session --type=method_call --dest=com.distillery.App /com/distillery/App com.distillery.App.ToggleFloatingMemo`
</details>

---

## 기술 스택

```
Frontend    Svelte 5 · SvelteKit · TypeScript · Tailwind CSS · DaisyUI
Backend     Rust · SQLite (FTS5) · reqwest
Desktop     Tauri 2
```

---

## 시작하기

### 준비물

- [Bun](https://bun.sh) (또는 Node.js)
- [Rust toolchain](https://rustup.rs)
- [Tauri 2 prerequisites](https://v2.tauri.app/start/prerequisites/)

### 설치

```bash
bun install

cp src-tauri/.env.sample src-tauri/.env
# src-tauri/.env 에서 API_BASE_URL, WEB_BASE_URL 설정
```

### 개발

```bash
bun run tauri dev
```

### 빌드

```bash
bun run tauri build
```

### 릴리즈 (포크 시)

1. 서명 키 생성:

```bash
bun tauri signer generate -w ~/.tauri/distillery.key
```

2. GitHub **Repository Secrets** 등록 (Settings → Secrets and variables → Actions):

| Secret | 값 |
|--------|---|
| `API_BASE_URL` | API 서버 URL |
| `WEB_BASE_URL` | 웹 앱 URL |
| `TAURI_SIGNING_PRIVATE_KEY` | 생성된 `.key` 파일의 내용 |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | 키 생성 시 입력한 비밀번호 |

3. 버전 태그를 푸시하면 빌드가 시작됩니다:

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub에 Linux, macOS, Windows 빌드가 포함된 draft release가 생성됩니다.

---

## 프로젝트 구조

```
src/
├── routes/              # 페이지 (몰팅, 몰트하우스, 스틸, 드로우백, 설정, 도움말)
├── lib/
│   ├── components/      # Svelte 컴포넌트
│   ├── stores/          # 상태 관리 (auth, malts, settings)
│   ├── i18n/            # 다국어 (한국어, English)
│   └── types.ts         # 타입 정의
src-tauri/
├── src/
│   ├── commands.rs      # Tauri IPC 명령
│   ├── db.rs            # SQLite 데이터베이스
│   ├── auth.rs          # OTP 인증
│   ├── api.rs           # 서버 API 통신
│   └── models.rs        # 데이터 모델
```

---

<p align="center"><sub>pitch black, amber-lit.</sub></p>
