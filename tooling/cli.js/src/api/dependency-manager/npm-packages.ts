// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

import { ManagementType, Result } from './types'
import {
  getNpmLatestVersion,
  getNpmPackageVersion,
  installNpmPackage,
  installNpmDevPackage,
  updateNpmPackage,
  semverLt,
  useYarn
} from './util'
import logger from '../../helpers/logger'
import { resolve } from '../../helpers/app-paths'
import inquirer from 'inquirer'
import { existsSync } from 'fs'
import { sync as crossSpawnSync } from 'cross-spawn'

const log = logger('dependency:npm-packages')

async function manageDependencies(
  managementType: ManagementType,
  dependencies: string[]
): Promise<Result> {
  const installedDeps = []
  const updatedDeps = []

  const npmChild = crossSpawnSync('npm', ['--version'])
  const yarnChild = crossSpawnSync('yarn', ['--version'])
  if (
    (npmChild.status ?? npmChild.error) &&
    (yarnChild.status ?? yarnChild.error)
  ) {
    throw new Error(
      'must have `npm` or `yarn` installed to manage dependenices'
    )
  }

  if (existsSync(resolve.app('package.json'))) {
    for (const dependency of dependencies) {
      const currentVersion = getNpmPackageVersion(dependency)
      if (currentVersion === null) {
        log(`Installing ${dependency}...`)
        if (
          managementType === ManagementType.Install ||
          managementType === ManagementType.InstallDev
        ) {
          const packageManager = useYarn() ? 'YARN' : 'NPM'
          const inquired = (await inquirer.prompt([
            {
              type: 'confirm',
              name: 'answer',
              message: `[${packageManager}]: "Do you want to install ${dependency} ${
                managementType === ManagementType.InstallDev
                  ? 'as dev-dependency'
                  : ''
              }?"`,
              default: false
            }
          ])) as { answer: boolean }
          if (inquired.answer) {
            if (managementType === ManagementType.Install) {
              installNpmPackage(dependency)
            } else if (managementType === ManagementType.InstallDev) {
              installNpmDevPackage(dependency)
            }
            installedDeps.push(dependency)
          }
        }
      } else if (managementType === ManagementType.Update) {
        const latestVersion = getNpmLatestVersion(dependency)
        if (semverLt(currentVersion, latestVersion)) {
          const inquired = (await inquirer.prompt([
            {
              type: 'confirm',
              name: 'answer',
              message: `[NPM]: "${dependency}" latest version is ${latestVersion}. Do you want to update?`,
              default: false
            }
          ])) as { answer: boolean }
          if (inquired.answer) {
            log(`Updating ${dependency}...`)
            updateNpmPackage(dependency)
            updatedDeps.push(dependency)
          }
        } else {
          log(`"${dependency}" is up to date`)
        }
      } else {
        log(`"${dependency}" is already installed`)
      }
    }
  }

  const result: Result = new Map<ManagementType, string[]>()
  result.set(ManagementType.Install, installedDeps)
  result.set(ManagementType.Update, updatedDeps)

  return result
}

const dependencies = ['@tauri-apps/api', '@tauri-apps/cli']

async function install(): Promise<Result> {
  return await manageDependencies(ManagementType.Install, dependencies)
}

async function installThese(dependencies: string[]): Promise<Result> {
  return await manageDependencies(ManagementType.Install, dependencies)
}

async function installTheseDev(dependencies: string[]): Promise<Result> {
  return await manageDependencies(ManagementType.InstallDev, dependencies)
}

async function update(): Promise<Result> {
  return await manageDependencies(ManagementType.Update, dependencies)
}

export { install, installThese, installTheseDev, update }
