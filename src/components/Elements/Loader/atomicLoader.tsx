import clsx from 'clsx';
import React from 'react';

import './atom.css';

export function AtomicLoader(): JSX.Element {
  return (
    <div className="justify-center flex p-6 m-auto">
      <div className="la-ball-atom la-2x">
        <div className="satellite text-orange-300">
          {/* <div className="crater border rounded-full bg-orange-400" /> */}
        </div>
        <div className="satellite text-red-600" />
        <div className="satellite text-azure-600" />
        <div className="satellite text-green-600" />
      </div>
    </div>
  );
}
