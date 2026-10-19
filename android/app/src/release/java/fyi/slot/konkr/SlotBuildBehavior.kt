package fyi.slot.konkr

import android.view.KeyEvent

/**
 * Production KONKR behavior. Mac AVD key remapping and side-by-side
 * multiplayer QA isolation live exclusively in src/debug.
 */
@Suppress("UNUSED_PARAMETER")
internal object SlotBuildBehavior {
    // Android already reports KONKR's controller buttons as native key codes.
    // Handhelds without KONKR's round top button (e.g. Retroid Pocket Classic)
    // use a click of the right stick (R3) as the slot. cap button.
    fun emulatorButton(key: Int): Int? =
        if (key == KeyEvent.KEYCODE_BUTTON_THUMBR) KeyEvent.KEYCODE_BUTTON_MODE else null

    fun storagePolicy(
        packageName: String,
        savesRoot: String?,
        statesRoot: String?,
        shareGbaSaves: Boolean
    ): SlotStorageSelection = SlotStorageSelection(savesRoot, statesRoot, shareGbaSaves)

    fun externalFolderRestrictionMessage(packageName: String): String? = null
}
