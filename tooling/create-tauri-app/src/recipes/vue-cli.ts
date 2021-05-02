// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { join } from 'path'
import { shell } from '../shell'
import { Recipe } from '../types/recipe'

const completeLogMsg = `
  Your installation completed.
  To start, run yarn tauri:serve
`

const vuecli: Recipe = {
  descriptiveName: 'Vue CLI',
  shortName: 'vuecli',
  extraNpmDevDependencies: [],
  extraNpmDependencies: [],
  configUpdate: ({ cfg }) => cfg,
  preInit: async ({ cwd, cfg, ci, packageManager }) => {
    // Vue CLI creates the folder for you
    await shell(
      'npx',
      [
        '@vue/cli',
        'create',
        `${cfg.appName}`,
        '--packageManager',
        packageManager,
        ci ? '--default' : ''
      ],
      { cwd }
    )
    await shell(
      'npx',
      [
        '@vue/cli',
        'add',
        'tauri',
        '--appName',
        `${cfg.appName}`,
        '--windowTitle',
        `${cfg.windowTitle}`
      ],
      {
        cwd: join(cwd, cfg.appName)
      }
    )
  },
  postInit: async () => {
    console.log(completeLogMsg)
    return await Promise.resolve()
  }
}

export { vuecli }
