
# StrixSense Listener

## 💡 Ambient Connectivity & Developer Feedback Notifier for ASUS Laptops

StrixSense Listener is a high-performance, multi-threaded system utility written in **Rust** that provides real-time, visual feedback by leveraging ASUS keyboard backlights. It addresses the lack of high-visibility, ambient notification systems for critical connectivity status and, in its planned evolution, for immediate developer workflow feedback.

## 🚀 Features

### Current (Connectivity Monitoring)

  * **Real-time Bluetooth Monitoring:** Watches `bluetoothctl` output to detect connected and disconnected states for peripherals.
  * **Real-time Wi-Fi Monitoring:** Watches `nmcli monitor` output to detect when a primary network connection is established or lost.
  * **Color-Coded Feedback:** Each event triggers a unique, temporary color animation for instant status recognition.
      * 🟢 **Bluetooth Connected:** Lime Green (`32CD32`) Breathe Effect.
      * 🔴 **Bluetooth Disconnected:** Deep Red (`8B0000`) Breathe Effect.
      * 🔵 **Wi-Fi Connected:** Sky Blue (`87CEEB`) Breathe Effect.
      * 🟡 **Wi-Fi Disconnected:** Warning Gold (`FFD700`) Breathe Effect.
  * **Automatic Reset:** After a 3-second notification period, the keyboard automatically returns to the default **Indigo (`4B0082`)** static color.
  * **Non-Blocking Concurrency:** Utilizes native Rust threading (`std::thread`) to monitor both services simultaneously without blocking the main process or each other.

### Future (Developer Feedback System)

  * **Local REST API Server:** The Rust application will run a lightweight local HTTP server (e.g., using **Actix Web** or **Axum**) to receive status updates from extensions.
  * **VS Code Integration:** An extension will monitor code diagnostics and build/run task status, sending a **Success** or **Error** signal to the Rust server.
  * **Web Platform Integration:** A Chrome extension will monitor submission results on sites like LeetCode and Colab, triggering a light effect based on acceptance or failure.

## 🏗️ Architecture (Current)

StrixSense Listener currently employs a simple, robust, event-driven, multi-threaded design.

### Component Breakdown

| Component | Responsibility | Implementation Source |
| :--- | :--- | :--- |
| **`main.rs`** | Application entry point. Spawns the dedicated Bluetooth and Wi-Fi monitor threads. | `main.rs` |
| **Monitor Threads** | Spawns external utilities (`bluetoothctl`, `nmcli monitor`), reads their piped output, and uses string matching for event detection. | `monitors.rs` |
| **`lights` Module** | Utility functions (`set_keyboard_breathe`, `set_keyboard_static`) that abstract the hardware interface. | `lights.rs` |
| **Hardware Driver** | The **`asusctl`** command-line tool is called by the `lights` module to interface with the laptop's keyboard Aura lighting. | OS Utility |

### Data Flow

Event lines stream from external utilities $\rightarrow$ Monitor Threads $\rightarrow$ Light Functions $\rightarrow$ `asusctl` utility $\rightarrow$ Keyboard Hardware. The monitor threads continuously use `BufReader::lines()` to wait for new data, ensuring low CPU usage in an idle state.

## ⚙️ Setup and Installation

### Prerequisites

  * **Operating System:** Linux Distribution (e.g., Arch, Fedora, Ubuntu).
  * **Hardware:** ASUS ROG/TUF laptop supported by `asusctl`.
  * **Dependencies:**
      * **Rust Toolchain:** Stable version of Rust (required for `cargo`).
      * **`asusctl`:** Must be installed and configured to manage keyboard lighting.
      * **`bluetoothctl`** and **`nmcli`** (standard on most Linux systems).

### Build & Run Instructions

1.  **Clone the Repository:**

    ```bash
    git clone https://github.com/chan9an/ROG-Keyboard/
    cd StrixSense_Listener
    ```

2.  **Verify asusctl:**
    Ensure `asusctl` is functional by manually running a command. (e.g., `asusctl aura static -c FF0000`).

3.  **Build the Project:**
    Compile the optimized release binary:

    ```bash
    cargo build --release
    ```

4.  **Execute the Listener:**
    Run the compiled binary. The application will start monitoring events and run indefinitely.

    ```bash
    ./target/release/strixsense
    ```

### Testing the Application

  * **Bluetooth Test:** Connect or disconnect a paired device to observe the **Lime Green** or **Deep Red** effects.
  * **Wi-Fi Test:** Disable your network connection and then reconnect to observe the **Gold** then **Sky Blue** effects.

## 🧪 Future Vision: Developer Feedback System

This system evolution requires significant architectural changes but offers substantial user value.

### Implementation Plan

1.  **Phase 1: REST API Implementation**

      * Integrate a Rust web framework (e.g., Actix Web or Axum).
      * Set up an asynchronous server on a local port (e.g., `127.0.0.1:8080`).
      * Define a `POST /api/status` endpoint to accept `{"status": "success"}` or `{"status": "error"}` JSON payloads.
      * The existing monitor threads will be wrapped to run alongside the new API server within the same application process.

2.  **Phase 2: VS Code Extension**

      * Develop a VS Code extension (TypeScript/JavaScript).
      * Utilize the VS Code Extension API to listen for build/run task completion or code diagnostic changes.
      * On a successful run, send a POST request with `{"status": "success"}`.
      * On a failed run or detected error, send a POST request with `{"status": "error"}`.

3.  **Phase 3: Chrome Extension**

      * Develop a Chrome extension with a Content Script injected into target domains (e.g., LeetCode, Google Colab).
      * The script will monitor the DOM (Document Object Model) or network requests to detect submission status (e.g., "Accepted" vs. "Wrong Answer").
      * Send the corresponding `{"status": "success"}` or `{"status": "error"}` payload to the local Rust API.

### Technical Challenges & Limitations

  * **Network Access:** The user's system must allow loopback HTTP connections for the extensions to talk to the local Rust server.
  * **Web Scraper Fragility:** The Chrome extension will rely on the specific HTML structure of LeetCode/Colab, making it highly susceptible to UI changes on those platforms.
  * **Complexity:** The architecture shifts from a simple multi-threaded CLI utility to a more complex client-server model, requiring robust error handling for network communication.
