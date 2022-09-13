import clsx from 'clsx';
import React from 'react';

import './atom.css';

export function AtomicLoader(): JSX.Element {
  return (
    <div className="justify-center flex p-6 m-auto">
      <div className="la-ball-atom la-2x text-cyan-700">
        <div></div>
        <div></div>
        <div></div>
        <div></div>
      </div>
    </div>
  );
}
