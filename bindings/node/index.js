const ffi = require('ffi-napi');
const ref = require('ref-napi');

function libraryName() {
  if (process.env.CAMJONGUN_FFI_PATH) return process.env.CAMJONGUN_FFI_PATH;
  if (process.platform === 'win32') return 'camjongun_ffi';
  if (process.platform === 'darwin') return 'libcamjongun_ffi';
  return 'libcamjongun_ffi';
}

const lib = ffi.Library(libraryName(), {
  cju_runtime_init: ['int', []],
  cju_runtime_shutdown: ['void', []],
  cju_camera_ensure: ['int', ['string', 'pointer']],
  cju_camera_rename: ['int', ['string']],
  cju_camera_install: ['int', []],
  cju_camera_uninstall: ['int', []],
  cju_result_message: ['string', ['int']],
  cju_last_error: ['string', []],
});

function check(code) {
  if (code === 0) return;
  throw new Error(lib.cju_last_error() || lib.cju_result_message(code));
}

function init() {
  check(lib.cju_runtime_init());
}

function shutdown() {
  lib.cju_runtime_shutdown();
}

function ensureCamera(displayName) {
  const id = Buffer.alloc(64);
  check(lib.cju_camera_ensure(displayName, id));
  return id.toString('utf8').replace(/\0.*$/, '');
}

function renameCamera(displayName) {
  check(lib.cju_camera_rename(displayName));
}

function installCamera() {
  check(lib.cju_camera_install());
}

function uninstallCamera() {
  check(lib.cju_camera_uninstall());
}

module.exports = {
  init,
  shutdown,
  ensureCamera,
  renameCamera,
  installCamera,
  uninstallCamera,
};
