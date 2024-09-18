import init, * as tools from './pkg/aleo_testnet2_wasm_suite.js';

/** query selector @returns {Element} */
const $ = document.querySelector.bind(document);
/** query selector all @returns {Element[]} */
const $$ = q => Array.from(document.querySelectorAll(q));

/**
 * @param {Element} el
 * @param {boolean | null} isInvalid
 */
function setInvalid(el, isInvalid = true) {
  if (isInvalid === null) {
    el.removeAttribute('aria-invalid');
    return;
  }
  el.setAttribute('aria-invalid', isInvalid + '');
}

/** @param {Element} el */
const clearInvalid = el => setInvalid(el, null);

/** @typedef {({form: HTMLFormElement} | Record<string, HTMLInputElement>)} FormElements   */

/**
 * get the inputs from a form
 * @param {string} formId
 * @returns {FormElements}
 */
function formElems(formId) {
  const form = $('#' + formId);
  const elems = $$(`#${formId} input`);
  if (!form || elems.length === 0) return null;

  const entries = Object.fromEntries(
    elems.map(e => [e.getAttribute('name').split('_').slice(1).join('_'), e])
  );

  return { form, ...entries };
}

const debounce = (fn, duration = 250) => {
  let timeout;
  return function debounced() {
    clearTimeout(timeout);
    timeout = setTimeout(fn, duration);
  };
};

/**
 * @param {string} formId Form Id
 * @param {(inputs: Record<string, HTMLInputElement>) => {}} handler
 */
function formHandler(formId, handler) {
  const { form, ...inputs } = formElems(formId);
  if (!inputs) {
    console.error('invalid selectors for form', formId);
    return;
  }

  const invoker = debounce(() => {
    for (const key in inputs) {
      /** @type {HTMLInputElement} */
      const el = inputs[key];
      clearInvalid(el);
      if (el.hasAttribute('readonly')) {
        el.value = '';
      }
    }
    handler(inputs);
  });

  for (const key in inputs) {
    /** @type {HTMLInputElement} */
    const el = inputs[key];
    clearInvalid(el);

    if (el.hasAttribute('readonly')) {
      el.setAttribute('data-tooltip', 'Click to copy to clipboard');
      const copyDone = debounce(() => clearInvalid(el), 1000);

      // copy text to clipboard on click
      el.addEventListener('click', _event => {
        if (el.value.length === 0) return;

        el.select();
        el.setSelectionRange(0, 99999);

        try {
          navigator.clipboard.writeText(el.value);
          setInvalid(el, false);
        } catch (err) {
          console.log('error copying text to clipboard', err);
          setInvalid(el);
        }
        copyDone();
      });
    } else {
      el.addEventListener('change', invoker);
      el.addEventListener('keyup', invoker);
      el.addEventListener('paste', invoker);
    }
  }
}

init()
  .then(() => {
    formHandler('signForm', ({ message, privateKey, signature }) => {
      if (privateKey.value.length === 0) return;

      try {
        tools.testnet2_address(privateKey.value);
      } catch (err) {
        setInvalid(privateKey);
        signature.value = err.toString();
        return;
      }

      try {
        signature.value = tools.testnet2_sign(privateKey.value, message.value);
      } catch (err) {
        setInvalid(message);
        signature.value = err.toString();
        return;
      }
    });

    formHandler('verifyForm', ({ address, message, signature }) => {
      if (address.value.length === 0) return;
      if (!tools.check_testnet2_address(address.value)) {
        return setInvalid(address);
      }

      if (signature.value.length === 0) return;

      try {
        setInvalid(
          signature,
          !tools.testnet2_verify(address.value, message.value, signature.value)
        );
      } catch (err) {
        console.log('error verifying address:', err);
        setInvalid(signature);
      }
    });

    formHandler('deriveForm', ({ privateKey, address, address2 }) => {
      if (privateKey.value.length === 0) return;

      try {
        address2.value = tools.testnet2_address(privateKey.value);
      } catch (err) {
        setInvalid(address2);
        address2.value = err.toString();
      }

      try {
        address.value = tools.mainnet_address(privateKey.value);
      } catch (e) {
        setInvalid(address);
        address.value = e.toString();
      }
    });
  })
  .catch(e => {
    console.log(e);
    $('#wasmWarning').classList.remove('hidden');
  });
