import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";
import { logger } from "$lib/utils/logger";
import { BundleType, getBundleType } from "@tauri-apps/api/app";


/**
 * Downloads and installs the latest update. The app will be closed and relaunched after the update has been
 * installed.
 *
 * @param update The new update.
 */
export async function installUpdate(update: Update){
    logger.info(`Downloading new update (${update.version})`);
    await update.downloadAndInstall()
        .catch(err => logger.error(`Failed to install new update: ${err.message}`));
    logger.info(`Installed update (${update.version}), relaunching app...`);
    await relaunch();
}

export async function checkForUpdate(): Promise<Update | null>{
    logger.info("Checking for update...");
    // FIXME: check if it was distributed via the store, IS_EXTERNAL_DIST
    // TODO: save distribution information
    const bundleType = await getBundleType();
    const updatableBundles = [BundleType.AppImage,BundleType.Nsis,BundleType.App];
    if (!updatableBundles.includes(bundleType)){
        logger.info(`Bundle ${bundleType} not updatable`);
        return null;
    }

    const update = await check();
    if(update) {
        logger.info(`Found update, version ${update.version}`);
    } else{
        logger.info("No update found");
    }
    return update;
}

