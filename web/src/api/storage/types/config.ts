/**
 * This file was automatically generated by json-schema-to-typescript.
 * DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
 * and run json-schema-to-typescript to regenerate this file.
 */

/**
 * Alias used to reference a device.
 */
export type Alias = string;
export type DriveElement = NonPartitionedDrive | PartitionedDrive;
export type SearchElement = SimpleSearchAll | SimpleSearchByName | AdvancedSearch;
/**
 * Shortcut to match all devices if there is any (equivalent to specify no conditions and to skip the entry if no device is found).
 */
export type SimpleSearchAll = "*";
export type SimpleSearchByName = string;
/**
 * How to handle the section if the device is not found.
 */
export type SearchAction = "skip" | "error";
export type Encryption =
  | EncryptionLuks1
  | EncryptionLuks2
  | EncryptionPervasiveLuks2
  | EncryptionTPM
  | EncryptionSwap;
/**
 * Password to use when creating a new encryption device.
 */
export type EncryptionPassword = string;
/**
 * The value must be compatible with the --cipher argument of the command cryptsetup.
 */
export type EncryptionCipher = string;
/**
 * The value (in bits) has to be a multiple of 8. The possible key sizes are limited by the used cipher.
 */
export type EncryptionKeySize = number;
export type EncryptionPbkdFunction = "pbkdf2" | "argon2i" | "argon2id";
/**
 * Swap encryptions.
 */
export type EncryptionSwap = "protected_swap" | "secure_swap" | "random_swap";
export type FilesystemType = FilesystemTypeAny | FilesystemTypeBtrfs;
export type FilesystemTypeAny =
  | "bcachefs"
  | "btrfs"
  | "exfat"
  | "ext2"
  | "ext3"
  | "ext4"
  | "f2fs"
  | "jfs"
  | "nfs"
  | "nilfs2"
  | "ntfs"
  | "reiserfs"
  | "swap"
  | "tmpfs"
  | "vfat"
  | "xfs";
/**
 * How to mount the device.
 */
export type MountBy = "device" | "id" | "label" | "path" | "uuid";
/**
 * Partition table type.
 */
export type PtableType = "gpt" | "msdos" | "dasd";
export type PartitionElement =
  | SimpleVolumesGenerator
  | AdvancedPartitionsGenerator
  | RegularPartition
  | PartitionToDelete
  | PartitionToDeleteIfNeeded;
export type PartitionId = "linux" | "swap" | "lvm" | "raid" | "esp" | "prep" | "bios_boot";
export type Size = SizeValue | SizeTuple | SizeRange;
export type SizeValue = SizeString | SizeBytes;
/**
 * Human readable size.
 */
export type SizeString = string;
/**
 * Size in bytes.
 */
export type SizeBytes = number;
/**
 * Lower size limit and optionally upper size limit.
 *
 * @minItems 1
 * @maxItems 2
 */
export type SizeTuple = [SizeValueWithCurrent] | [SizeValueWithCurrent, SizeValueWithCurrent];
export type SizeValueWithCurrent = SizeValue | SizeCurrent;
/**
 * The current size of the device.
 */
export type SizeCurrent = "current";
export type PhysicalVolumeElement =
  | Alias
  | SimplePhysicalVolumesGenerator
  | AdvancedPhysicalVolumesGenerator;
export type LogicalVolumeElement =
  | SimpleVolumesGenerator
  | AdvancedLogicalVolumesGenerator
  | LogicalVolume
  | ThinPoolLogicalVolume
  | ThinLogicalVolume;
/**
 * Number of stripes.
 */
export type LogicalVolumeStripes = number;
export type MdRaidElement = NonPartitionedMdRaid | PartitionedMdRaid;
/**
 * MD base name.
 */
export type MdRaidName = string;
export type MDLevel = "raid0" | "raid1" | "raid5" | "raid6" | "raid10";
/**
 * Only applies to raid5, raid6 and raid10
 */
export type MDParity =
  | "left_asymmetric"
  | "left_symmetric"
  | "right_asymmetric"
  | "right_symmetric"
  | "first"
  | "last"
  | "near_2"
  | "offset_2"
  | "far_2"
  | "near_3"
  | "offset_3"
  | "far_3";
/**
 * Devices used by the MD RAID.
 */
export type MdRaidDevices = Alias[];

/**
 * Storage config.
 */
export interface Config {
  boot?: Boot;
  /**
   * Drives (disks, BIOS RAIDs and multipath devices).
   */
  drives?: DriveElement[];
  /**
   * LVM volume groups.
   */
  volumeGroups?: VolumeGroup[];
  /**
   * MD RAIDs.
   */
  mdRaids?: MdRaidElement[];
}
/**
 * Allows configuring boot partitions automatically.
 */
export interface Boot {
  /**
   * Whether to configure partitions for booting.
   */
  configure: boolean;
  device?: Alias;
}
/**
 * Drive without a partition table (e.g., directly formatted).
 */
export interface NonPartitionedDrive {
  search?: SearchElement;
  alias?: Alias;
  encryption?: Encryption;
  filesystem?: Filesystem;
}
/**
 * Advanced options for searching devices.
 */
export interface AdvancedSearch {
  condition?: SearchCondition;
  /**
   * Maximum devices to match.
   */
  max?: number;
  ifNotFound?: SearchAction;
}
export interface SearchCondition {
  name: SimpleSearchByName;
}
/**
 * LUKS1 encryption.
 */
export interface EncryptionLuks1 {
  luks1: {
    password: EncryptionPassword;
    cipher?: EncryptionCipher;
    keySize?: EncryptionKeySize;
  };
}
/**
 * LUKS2 encryption.
 */
export interface EncryptionLuks2 {
  luks2: {
    password: EncryptionPassword;
    cipher?: EncryptionCipher;
    keySize?: EncryptionKeySize;
    pbkdFunction?: EncryptionPbkdFunction;
    /**
     * LUKS2 label.
     */
    label?: string;
  };
}
/**
 * LUKS2 pervasive encryption.
 */
export interface EncryptionPervasiveLuks2 {
  pervasiveLuks2: {
    password: EncryptionPassword;
  };
}
/**
 * TPM-Based Full Disk Encrytion.
 */
export interface EncryptionTPM {
  tpmFde: {
    password: EncryptionPassword;
  };
}
export interface Filesystem {
  /**
   * Try to reuse the existing file system. In some cases the file system could not be reused, for example, if the device is re-encrypted.
   */
  reuseIfPossible?: boolean;
  type?: FilesystemType;
  /**
   * File system label.
   */
  label?: string;
  /**
   * Mount path.
   */
  path?: string;
  mountBy?: MountBy;
  /**
   * Options for creating the file system.
   */
  mkfsOptions?: string[];
  /**
   * Options to add to the fourth field of fstab.
   */
  mountOptions?: string[];
}
/**
 * Btrfs file system.
 */
export interface FilesystemTypeBtrfs {
  btrfs: {
    /**
     * Whether to configrue Btrfs snapshots.
     */
    snapshots?: boolean;
  };
}
export interface PartitionedDrive {
  search?: SearchElement;
  alias?: Alias;
  ptableType?: PtableType;
  partitions: PartitionElement[];
}
/**
 * Automatically creates the default or mandatory volumes configured by the selected product.
 */
export interface SimpleVolumesGenerator {
  generate: "default" | "mandatory";
}
/**
 * Creates the default or mandatory partitions configured by the selected product.
 */
export interface AdvancedPartitionsGenerator {
  generate: {
    partitions: "default" | "mandatory";
    encryption?: Encryption;
  };
}
export interface RegularPartition {
  search?: SearchElement;
  alias?: Alias;
  id?: PartitionId;
  size?: Size;
  encryption?: Encryption;
  filesystem?: Filesystem;
}
/**
 * Size range.
 */
export interface SizeRange {
  min: SizeValueWithCurrent;
  max?: SizeValueWithCurrent;
}
export interface PartitionToDelete {
  search: SearchElement;
  /**
   * Delete the partition.
   */
  delete: true;
}
export interface PartitionToDeleteIfNeeded {
  search: SearchElement;
  /**
   * Delete the partition if needed to make space.
   */
  deleteIfNeeded: true;
  size?: Size;
}
/**
 * LVM volume group.
 */
export interface VolumeGroup {
  /**
   * Volume group name.
   */
  name: string;
  extentSize?: SizeValue;
  /**
   * Devices to use as physical volumes.
   */
  physicalVolumes?: PhysicalVolumeElement[];
  logicalVolumes?: LogicalVolumeElement[];
}
/**
 * Automatically creates the needed physical volumes in the indicated devices.
 */
export interface SimplePhysicalVolumesGenerator {
  generate: Alias[];
}
/**
 * Automatically creates the needed physical volumes in the indicated devices.
 */
export interface AdvancedPhysicalVolumesGenerator {
  generate: {
    targetDevices: Alias[];
    encryption?: Encryption;
  };
}
/**
 * Automatically creates the default or mandatory logical volumes configured by the selected product.
 */
export interface AdvancedLogicalVolumesGenerator {
  generate: {
    logicalVolumes: "default" | "mandatory";
    encryption?: Encryption;
    stripes?: LogicalVolumeStripes;
    stripeSize?: SizeValue;
  };
}
export interface LogicalVolume {
  /**
   * Logical volume name.
   */
  name?: string;
  size?: Size;
  stripes?: LogicalVolumeStripes;
  stripeSize?: SizeValue;
  encryption?: Encryption;
  filesystem?: Filesystem;
}
export interface ThinPoolLogicalVolume {
  /**
   * LVM thin pool.
   */
  pool: true;
  alias?: Alias;
  /**
   * Logical volume name.
   */
  name?: string;
  size?: Size;
  stripes?: LogicalVolumeStripes;
  stripeSize?: SizeValue;
  encryption?: Encryption;
}
export interface ThinLogicalVolume {
  /**
   * Thin logical volume name.
   */
  name?: string;
  size?: Size;
  usedPool: Alias;
  encryption?: Encryption;
  filesystem?: Filesystem;
}
/**
 * MD RAID without a partition table (e.g., directly formatted).
 */
export interface NonPartitionedMdRaid {
  search?: SearchElement;
  alias?: Alias;
  name?: MdRaidName;
  level?: MDLevel;
  parity?: MDParity;
  chunkSize?: SizeValue;
  devices?: MdRaidDevices;
  encryption?: Encryption;
  filesystem?: Filesystem;
}
export interface PartitionedMdRaid {
  search?: SearchElement;
  alias?: Alias;
  name?: MdRaidName;
  level?: MDLevel;
  parity?: MDParity;
  chunkSize?: SizeValue;
  devices?: MdRaidDevices;
  ptableType?: PtableType;
  partitions: PartitionElement[];
}
